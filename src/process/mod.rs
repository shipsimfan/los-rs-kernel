//#![allow(dead_code)]

mod control;
mod daemon;
mod loader;
mod process;
mod queue;
mod thread;

use crate::{
    error,
    filesystem::{self, open_directory, DirectoryDescriptor},
    map::{Mappable, INVALID_ID},
    memory::KERNEL_VMA,
    session::{self, SessionBox},
};
use alloc::{string::String, vec::Vec};
use core::usize;

pub type Thread = thread::Thread;
pub type Process = process::Process;

pub type ThreadQueue = queue::ThreadQueue;

pub type ThreadFunc = fn() -> isize;
pub type ThreadFuncContext = fn(context: usize) -> isize;

#[repr(packed(1))]
struct UserspaceContext {
    argc: usize,
    argv: *const *const u8,
    envp: *const *const u8,
    stdio: *const CStandardIO,
    tls_size: usize,
    tls_align: usize,
}

#[repr(packed(1))]
pub struct CStandardIO {
    stdout_type: usize,
    stdout_desc: isize,
    stderr_type: usize,
    stderr_desc: isize,
    stdin_type: usize,
    stdin_desc: isize,
}

pub struct StandardIO {
    stdout: StandardIOType,
    stderr: StandardIOType,
    stdin: StandardIOType,
}

pub enum StandardIOType {
    None,
    Console,
    File(isize),
}

const TLS_LOCATION: usize = 0x700000000000;
const USERSPACE_CONTEXT_LOCATION: *mut UserspaceContext =
    (TLS_LOCATION - core::mem::size_of::<UserspaceContext>()) as *mut UserspaceContext;

const STDIO_TYPE_NONE: usize = 0;
const STDIO_TYPE_CONSOLE: usize = 1;
const STDIO_TYPE_FILE: usize = 2;

static mut THREAD_CONTROL: control::ThreadControl = control::ThreadControl::new();

extern "C" {
    #[allow(improper_ctypes)]
    fn perform_yield(
        save_location: *const usize,
        load_location: *const usize,
        next_thread: *mut Thread,
    );
}

#[no_mangle]
unsafe extern "C" fn switch_thread(next_thread: *mut Thread) {
    match THREAD_CONTROL.get_current_thread_mut() {
        None => {}
        Some(current_thread) => {
            if !current_thread.in_queue() {
                let current_process = current_thread.get_process_mut();
                if current_process.remove_thread(current_thread.id()) {
                    let current_process_id = current_process.id();
                    match current_process.get_session_mut() {
                        None => daemon::remove_process(current_process_id),
                        Some(session) => (*session.as_ptr()).remove_process(current_process_id),
                    }
                }
            }
        }
    }

    THREAD_CONTROL.set_current_thread(next_thread);
}

fn do_execute(
    filepath: &str,
    args: Vec<String>,
    environment: Vec<String>,
    mut standard_io: StandardIO,
    session_lock: Option<SessionBox>,
) -> error::Result<isize> {
    // Load the executable
    let buffer = filesystem::read(filepath)?;

    // Parse the elf header
    let entry = loader::verify_executable(&buffer)?;
    if entry >= KERNEL_VMA {
        return Err(error::Status::ArgumentSecurity);
    }

    // Get the current process
    let current_process = get_current_thread_mut().get_process_mut();

    // Figure out working directory
    let working_directory = match current_process.get_current_working_directory() {
        None => {
            let mut iter = filepath.split(|c| -> bool { c == '\\' || c == '/' });
            iter.next_back();

            let mut path = String::new();
            while let Some(part) = iter.next() {
                path.push_str(part);
                path.push('/');
            }

            path.pop();

            open_directory(&path)?
        }
        Some(working_directory) => DirectoryDescriptor::new(working_directory.get_directory()),
    };

    // Create a process
    let pid = match &session_lock {
        Some(session_lock) => session_lock.lock().create_process(
            entry,
            USERSPACE_CONTEXT_LOCATION as usize,
            Some(working_directory),
            session_lock.clone(),
        ),
        None => daemon::create_process(
            entry,
            USERSPACE_CONTEXT_LOCATION as usize,
            Some(working_directory),
        ),
    };

    // Get new process
    let new_process = match &session_lock {
        Some(session_lock) => {
            let mut session = session_lock.lock();
            unsafe { &mut *(session.get_process_mut(pid).unwrap() as *mut Process) }
        }
        None => {
            let mut daemon = daemon::get_daemon().lock();
            unsafe { &mut *(daemon.get_mut(pid).unwrap() as *mut Process) }
        }
    };

    // Copy stdio descriptors
    standard_io.copy_descriptors(current_process, new_process)?;

    unsafe { asm!("cli") };

    // Switch to new process address space
    new_process.set_address_space_as_current();

    // Copy the executable into the address space
    let (tls_size, tls_align) = loader::load_executable(&buffer, TLS_LOCATION as *mut u8)?;

    // Copy arguments, environment variables, and stdio into the address space
    unsafe {
        // Prepare locations
        let context_location = USERSPACE_CONTEXT_LOCATION as usize;
        let stdio_location = context_location + core::mem::size_of::<UserspaceContext>();
        let arg_list_start = stdio_location + core::mem::size_of::<CStandardIO>();
        let env_list_start = arg_list_start + (core::mem::size_of::<*mut u8>() * (args.len() + 1));
        let args_start =
            env_list_start + (core::mem::size_of::<*mut u8>() * (environment.len() + 1));
        let mut arg_list = arg_list_start as *mut *mut u8;
        let mut env_list = env_list_start as *mut *mut u8;
        let stdio = stdio_location as *mut CStandardIO;

        // Copy context
        let context = context_location as *mut UserspaceContext;
        *context = UserspaceContext {
            argc: args.len(),
            argv: arg_list as *const *const u8,
            envp: env_list as *const *const u8,
            tls_size: tls_size,
            tls_align: tls_align,
            stdio: stdio_location as *const CStandardIO,
        };

        // Copy arguments
        let mut ptr = args_start as *mut u8;
        for arg in args {
            *arg_list = ptr;
            arg_list = arg_list.add(1);

            for byte in arg.as_bytes() {
                *ptr = *byte;
                ptr = ptr.add(1);
            }

            *ptr = 0;
            ptr = ptr.add(1);
        }

        *arg_list = core::ptr::null_mut();

        // Copy environment variables
        for env in environment {
            *env_list = ptr;
            env_list = env_list.add(1);

            for byte in env.as_bytes() {
                *ptr = *byte;
                ptr = ptr.add(1);
            }

            *ptr = 0;
            ptr = ptr.add(1);
        }

        *env_list = core::ptr::null_mut();

        // Copy stdio
        *stdio = standard_io.to_c_stdio();

        // Queue the new process
        queue_thread_cli(new_process.get_thread_mut(0).unwrap());
    }

    // Return to current process address space
    current_process.set_address_space_as_current();

    // Return the process id
    unsafe { asm!("sti") };
    Ok(pid)
}

pub fn execute_session(
    filepath: &str,
    args: Vec<String>,
    environment: Vec<String>,
    standard_io: StandardIO,
    sid: isize,
) -> error::Result<isize> {
    if sid == 0 {
        do_execute(filepath, args, environment, standard_io, None)
    } else {
        match session::get_session_mut(sid) {
            None => Err(error::Status::InvalidSession),
            Some(session) => do_execute(filepath, args, environment, standard_io, Some(session)),
        }
    }
}

pub fn execute(
    filepath: &str,
    args: Vec<String>,
    environment: Vec<String>,
    standard_io: StandardIO,
) -> error::Result<isize> {
    let current_process = get_current_thread_mut().get_process_mut();
    let session = current_process.get_session_mut();
    do_execute(filepath, args, environment, standard_io, session)
}

pub fn create_thread(entry: ThreadFunc) -> isize {
    create_thread_raw(entry as usize, 0)
}

pub fn create_thread_raw(entry: usize, context: usize) -> isize {
    let current_process = get_current_thread_mut().get_process_mut();
    let tid = current_process.create_thread(entry, context);
    queue_thread(current_process.get_thread_mut(tid).unwrap());
    tid
}

pub fn create_process(entry: ThreadFunc, working_directory: Option<DirectoryDescriptor>) -> isize {
    match get_current_thread_mut_option() {
        Some(current_thread) => {
            let current_process = current_thread.get_process_mut();
            match current_process.get_session_mut() {
                None => {
                    let pid = daemon::create_process(entry as usize, 0, working_directory);
                    queue_thread(
                        daemon::get_daemon()
                            .lock()
                            .get_mut(pid)
                            .unwrap()
                            .get_thread_mut(0)
                            .unwrap(),
                    );
                    pid
                }
                Some(current_session) => {
                    let pid = current_session.lock().create_process(
                        entry as usize,
                        0,
                        working_directory,
                        current_session.clone(),
                    );
                    queue_thread(
                        current_session
                            .lock()
                            .get_process_mut(pid)
                            .unwrap()
                            .get_thread_mut(0)
                            .unwrap(),
                    );
                    pid
                }
            }
        }
        None => {
            let pid = daemon::create_process(entry as usize, 0, working_directory);
            queue_thread(
                daemon::get_daemon()
                    .lock()
                    .get_mut(pid)
                    .unwrap()
                    .get_thread_mut(0)
                    .unwrap(),
            );
            pid
        }
    }
}

pub unsafe fn queue_thread_cli(thread: &mut Thread) {
    THREAD_CONTROL.queue_execution(thread);
}

pub fn queue_thread(thread: &mut Thread) {
    unsafe {
        asm!("cli");
        queue_thread_cli(thread);
        asm!("sti");
    }
}

pub fn queue_and_yield() {
    queue_thread(get_current_thread_mut());

    yield_thread();
}

pub fn yield_thread() {
    loop {
        unsafe {
            asm!("cli");
            while let Some(next_thread) = THREAD_CONTROL.get_next_thread() {
                let default_location: usize = 0;
                let (save_location, load_location) = {
                    (
                        match THREAD_CONTROL.get_current_thread_mut() {
                            None => &default_location as *const usize,
                            Some(current_thread) => {
                                current_thread.save_float();

                                current_thread.get_stack_pointer_location()
                            }
                        },
                        {
                            next_thread.get_process().set_address_space_as_current();
                            next_thread.load_float();
                            next_thread.set_interrupt_stack();
                            next_thread.get_stack_pointer_location()
                        },
                    )
                };

                perform_yield(save_location, load_location, next_thread);

                return;
            }
            {
                asm!("sti; hlt")
            }
        }
    }
}

pub fn wait_thread(tid: isize) -> isize {
    let current_thread = get_current_thread_mut();
    match current_thread.get_process_mut().get_thread_mut(tid) {
        None => return isize::MIN,
        Some(thread) => {
            unsafe { asm!("cli") };
            thread.insert_into_exit_queue(current_thread)
        }
    }

    yield_thread();

    current_thread.get_queue_data()
}

pub fn wait_process(pid: isize) -> isize {
    let current_thread = get_current_thread_mut();
    match current_thread.get_process_mut().get_session_mut() {
        None => match daemon::get_daemon().lock().get_mut(pid) {
            None => return isize::MIN,
            Some(process) => process.insert_into_exit_queue(current_thread),
        },
        Some(session) => match session.lock().get_process_mut(pid) {
            None => return isize::MIN,
            Some(process) => process.insert_into_exit_queue(current_thread),
        },
    }

    yield_thread();

    current_thread.get_queue_data()
}

pub fn exit_thread(exit_status: isize) -> ! {
    unsafe {
        asm!("cli");
        let current_thread = get_current_thread_mut_cli();

        current_thread.pre_exit(exit_status);
        current_thread.get_process_mut().pre_exit(exit_status);

        current_thread.clear_queue();
        yield_thread();
        panic!("Returned to thread after exit!");
    }
}

pub fn exit_process(exit_status: isize) -> ! {
    unsafe {
        asm!("cli");
        let current_thread = get_current_thread_mut_cli();
        let current_process = current_thread.get_process_mut();

        current_process.kill_threads(current_thread.id());

        exit_thread(exit_status);
    }
}

pub fn kill_thread(tid: isize) {
    unsafe {
        asm!("cli");
        let current_thread = get_current_thread_mut_cli();
        let current_process = current_thread.get_process_mut();

        match current_process.get_thread_mut(tid) {
            Some(thread) => {
                if thread as *mut _ == current_thread as *mut _ {
                    exit_thread(128);
                } else {
                    current_process.remove_thread(tid);
                }
            }
            None => {}
        }
        asm!("sti");
    }
}

pub fn kill_process(pid: isize) {
    let current_process = get_current_thread_mut().get_process_mut();
    let session_lock = match current_process.get_session_mut() {
        Some(session) => session,
        None => return daemon::kill_process(pid),
    };
    let mut session = session_lock.lock();

    unsafe {
        asm!("cli");

        let remove = match session.get_process_mut(pid) {
            Some(process) => {
                if process as *const _ == current_process as *const _ {
                    exit_process(128);
                } else {
                    process.kill_threads(INVALID_ID);
                    process.pre_exit(128);
                    true
                }
            }
            None => false,
        };

        if remove {
            session.remove_process(pid);
        }

        asm!("sti");
    }
}

pub fn get_current_thread() -> &'static Thread {
    unsafe {
        asm!("cli");
        let ret = get_current_thread_cli();
        asm!("sti");
        ret
    }
}

pub unsafe fn get_current_thread_cli() -> &'static Thread {
    get_current_thread_option_cli().expect("No current thread when one required!")
}

pub unsafe fn get_current_thread_option_cli() -> Option<&'static Thread> {
    THREAD_CONTROL.get_current_thread()
}

pub fn get_current_thread_option() -> Option<&'static Thread> {
    unsafe {
        asm!("cli");
        let ret = get_current_thread_option_cli();
        asm!("sti");
        ret
    }
}

pub fn get_current_thread_mut() -> &'static mut Thread {
    unsafe {
        asm!("cli");
        let ret = get_current_thread_mut_cli();
        asm!("sti");
        ret
    }
}

pub unsafe fn get_current_thread_mut_cli() -> &'static mut Thread {
    get_current_thread_mut_option_cli().expect("No current thread when one required!")
}

pub fn get_current_thread_mut_option() -> Option<&'static mut Thread> {
    unsafe {
        asm!("cli");
        let ret = get_current_thread_mut_option_cli();
        asm!("sti");
        ret
    }
}

pub unsafe fn get_current_thread_mut_option_cli() -> Option<&'static mut Thread> {
    THREAD_CONTROL.get_current_thread_mut()
}

pub fn preempt() {
    unsafe {
        if !THREAD_CONTROL.is_next_thread() {
            return;
        }

        if get_current_thread_option_cli().is_none() {
            return;
        }
    }

    queue_and_yield();
}

impl StandardIO {
    pub fn new(stdout: StandardIOType, stderr: StandardIOType, stdin: StandardIOType) -> Self {
        StandardIO {
            stdout,
            stderr,
            stdin,
        }
    }

    pub fn copy_descriptors(
        &mut self,
        current_process: &mut Process,
        new_process: &mut Process,
    ) -> error::Result<()> {
        self.stdout.copy_descriptors(current_process, new_process)?;
        self.stderr.copy_descriptors(current_process, new_process)?;
        self.stdin.copy_descriptors(current_process, new_process)
    }

    pub fn to_c_stdio(self) -> CStandardIO {
        let (stdout_type, stdout_desc) = self.stdout.to_c_stdio();
        let (stderr_type, stderr_desc) = self.stderr.to_c_stdio();
        let (stdin_type, stdin_desc) = self.stdin.to_c_stdio();

        CStandardIO {
            stdout_type,
            stdout_desc,
            stderr_type,
            stderr_desc,
            stdin_type,
            stdin_desc,
        }
    }
}

impl StandardIOType {
    pub fn to_c_stdio(self) -> (usize, isize) {
        match self {
            StandardIOType::None => (STDIO_TYPE_NONE, 0),
            StandardIOType::Console => (STDIO_TYPE_CONSOLE, 0),
            StandardIOType::File(fd) => (STDIO_TYPE_FILE, fd),
        }
    }

    pub fn copy_descriptors(
        &mut self,
        current_process: &mut Process,
        new_process: &mut Process,
    ) -> error::Result<()> {
        match self {
            StandardIOType::None | &mut StandardIOType::Console => Ok(()),
            StandardIOType::File(fd) => {
                let descriptor = current_process.get_file(*fd)?;
                *fd = new_process.clone_file(descriptor);
                Ok(())
            }
        }
    }
}

impl CStandardIO {
    pub fn to_stdio(&self) -> error::Result<StandardIO> {
        Ok(StandardIO::new(
            CStandardIO::parse(self.stdout_type, self.stdout_desc)?,
            CStandardIO::parse(self.stderr_type, self.stderr_desc)?,
            CStandardIO::parse(self.stdin_type, self.stdin_desc)?,
        ))
    }

    fn parse(class: usize, desc: isize) -> error::Result<StandardIOType> {
        match class {
            STDIO_TYPE_NONE => Ok(StandardIOType::None),
            STDIO_TYPE_CONSOLE => Ok(StandardIOType::Console),
            STDIO_TYPE_FILE => Ok(StandardIOType::File(desc)),
            _ => Err(error::Status::OutOfRange),
        }
    }
}
