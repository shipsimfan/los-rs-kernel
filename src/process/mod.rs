//#![allow(dead_code)]

mod control;
mod daemon;
mod loader;
mod process;
mod queue;
mod thread;

use crate::{
    critical::{self, CriticalLock},
    error,
    filesystem::{self, open_directory, DirectoryDescriptor},
    ipc::Signals,
    locks::Spinlock,
    map::{Mappable, INVALID_ID},
    memory::KERNEL_VMA,
    session::get_session,
};
use alloc::{borrow::ToOwned, string::String, vec::Vec};
use core::{arch::asm, usize};

pub use daemon::get_daemon_process;
pub use process::*;
pub use queue::ThreadQueue;
pub use thread::*;

#[repr(packed(1))]
#[allow(unused)]
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

static THREAD_CONTROL: CriticalLock<control::ThreadControl> =
    CriticalLock::new(control::ThreadControl::new());
static NEXT_THREAD: Spinlock<Option<(ThreadOwner, Option<CurrentQueue>)>> = Spinlock::new(None);

extern "C" {
    #[allow(improper_ctypes)]
    fn perform_yield(
        save_location: *const usize,
        load_location: *const usize,
        new_kernel_stack_base: usize,
    );
}

#[no_mangle]
unsafe extern "C" fn switch_thread() {
    let (new_thread, new_queue) = NEXT_THREAD.lock().take().unwrap();
    let old_thread = THREAD_CONTROL
        .lock()
        .set_current_thread(new_thread, new_queue);
    drop(old_thread);
}

fn do_execute(
    filepath: &str,
    args: Vec<String>,
    environment: Vec<String>,
    mut standard_io: StandardIO,
    session_id: Option<isize>,
    inherit_signals: bool,
) -> error::Result<isize> {
    // Load the executable
    let buffer = filesystem::read(filepath)?;

    let name = filepath.split(&['/', '\\']).last().unwrap();

    // Parse the elf header
    let entry = loader::verify_executable(&buffer)?;
    if entry >= KERNEL_VMA {
        return Err(error::Status::ArgumentSecurity);
    }

    // Get the current process
    let current_process = get_current_thread().process().unwrap();

    // Figure out working directory & signals
    let (working_directory, signals) = {
        let process_lock = current_process.upgrade().unwrap();
        let mut process = process_lock.lock();
        (
            match process.current_working_directory() {
                None => {
                    let mut iter = filepath.split(|c| -> bool { c == '\\' || c == '/' });
                    iter.next_back();

                    let mut path = String::new();
                    while let Some(part) = iter.next() {
                        path.push_str(part);
                        path.push('/');
                    }

                    path.pop();

                    open_directory(&path, None)?
                }
                Some(working_directory) => {
                    DirectoryDescriptor::new(working_directory.get_directory())
                }
            },
            if inherit_signals {
                process.signals().clone()
            } else {
                Signals::new()
            },
        )
    };

    // Create a process
    let new_thread = match session_id {
        Some(session_id) => match get_session(session_id) {
            Some(session) => session.lock().create_process(
                entry,
                USERSPACE_CONTEXT_LOCATION as usize,
                Some(working_directory),
                name.to_owned(),
                signals,
            ),
            None => return Err(error::Status::InvalidSession),
        },
        None => daemon::create_process(
            entry,
            USERSPACE_CONTEXT_LOCATION as usize,
            Some(working_directory),
            name.to_owned(),
            signals,
        ),
    };

    let new_process = new_thread.process();

    // Copy stdio descriptors
    standard_io.copy_descriptors(&current_process, &new_process)?;

    let critical_state = unsafe { crate::critical::enter_local() };

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
        queue_thread(new_thread);
    }

    // Return to current process address space
    current_process.set_address_space_as_current();

    // Return the process id
    unsafe { crate::critical::leave_local(critical_state) };
    Ok(new_process.id())
}

pub fn execute_session(
    filepath: &str,
    args: Vec<String>,
    environment: Vec<String>,
    standard_io: StandardIO,
    session_id: Option<isize>,
    inherit_signals: bool,
) -> error::Result<isize> {
    do_execute(
        filepath,
        args,
        environment,
        standard_io,
        session_id,
        inherit_signals,
    )
}

pub fn execute(
    filepath: &str,
    args: Vec<String>,
    environment: Vec<String>,
    standard_io: StandardIO,
    inherit_signals: bool,
) -> error::Result<isize> {
    let current_process = get_current_thread().process().unwrap();
    let session = current_process.session_id();
    do_execute(
        filepath,
        args,
        environment,
        standard_io,
        session,
        inherit_signals,
    )
}

#[allow(dead_code)]
pub fn create_thread(entry: ThreadFunc) -> isize {
    create_thread_raw(entry as usize, 0)
}

pub fn create_thread_raw(entry: usize, context: usize) -> isize {
    let current_process = get_current_thread().process().unwrap();
    let new_thread = current_process.create_thread(entry, context).unwrap();
    let tid = new_thread.id();
    queue_thread(new_thread);
    tid
}

pub fn create_process(
    entry: ThreadFunc,
    working_directory: Option<DirectoryDescriptor>,
    name: String,
) -> isize {
    match get_current_thread_option() {
        Some(current_thread) => {
            let current_process = current_thread.process().unwrap();
            match current_process.session_id() {
                None => {
                    let new_thread = daemon::create_process(
                        entry as usize,
                        0,
                        working_directory,
                        name,
                        current_process.signals().unwrap_or(Signals::new()),
                    );
                    let pid = new_thread.process().id();
                    queue_thread(new_thread);
                    pid
                }
                Some(session_id) => match get_session(session_id) {
                    Some(current_session) => {
                        let new_thread = current_session.lock().create_process(
                            entry as usize,
                            0,
                            working_directory,
                            name,
                            current_process.signals().unwrap_or(Signals::new()),
                        );
                        let pid = new_thread.process().id();
                        queue_thread(new_thread);
                        pid
                    }
                    None => panic!("Current session doesn't exist!"),
                },
            }
        }
        None => {
            let new_thread =
                daemon::create_process(entry as usize, 0, working_directory, name, Signals::new());
            let pid = new_thread.process().id();
            queue_thread(new_thread);
            pid
        }
    }
}

pub fn queue_thread(thread: ThreadOwner) {
    THREAD_CONTROL.lock().queue_execution(thread);
}

pub fn queue_and_yield(critical_state: Option<bool>) {
    let running_queue = THREAD_CONTROL.lock().get_current_queue();
    yield_thread(Some(running_queue), critical_state);
}

pub fn yield_thread(queue: Option<CurrentQueue>, critical_state: Option<bool>) {
    loop {
        unsafe {
            let critical_state = match critical_state {
                Some(state) => state,
                None => crate::critical::enter_local(),
            };
            let next_thread = THREAD_CONTROL.lock().get_next_thread();
            match next_thread {
                Some(next_thread) => {
                    let default_location: usize = 0;
                    let (save_location, (load_location, new_kernel_stack_base)) = {
                        (
                            match THREAD_CONTROL.lock().get_current_thread() {
                                None => &default_location as *const usize,
                                Some(current_thread) => {
                                    current_thread.save_float();

                                    match current_thread.get_stack_pointer_location() {
                                        Some(location) => location,
                                        None => &default_location as *const usize,
                                    }
                                }
                            },
                            {
                                next_thread.process().set_address_space_as_current();
                                next_thread.load_float();
                                next_thread.set_interrupt_stack();
                                (
                                    next_thread.get_stack_pointer_location(),
                                    next_thread.get_kernel_stack_base(),
                                )
                            },
                        )
                    };

                    (*NEXT_THREAD.lock()) = Some((next_thread, queue));
                    perform_yield(save_location, load_location, new_kernel_stack_base);

                    crate::critical::leave_local(critical_state);
                    return;
                }
                None => {
                    crate::critical::leave_local(critical_state);
                    asm!("hlt")
                }
            }
        }
    }
}

pub fn wait_thread(tid: isize) -> isize {
    let current_thread = get_current_thread();
    let exit_queue = match current_thread.process().unwrap().get_thread(tid) {
        None => return isize::MIN,
        Some(thread) => match thread.get_exit_queue() {
            Some(exit_queue) => exit_queue,
            None => return isize::MIN,
        },
    };

    yield_thread(Some(exit_queue), None);

    current_thread.get_queue_data().unwrap()
}

pub fn wait_process(pid: isize) -> isize {
    let current_thread = get_current_thread();
    let critical_state = unsafe { critical::enter_local() };
    let exit_queue = match current_thread.process().unwrap().session_id() {
        None => match daemon::get_daemon_process(pid) {
            None => return isize::MIN,
            Some(process) => match process.get_exit_queue() {
                Some(queue) => queue,
                None => return isize::MIN,
            },
        },
        Some(session_id) => match get_session(session_id) {
            Some(session) => match session.lock().get_process(pid) {
                None => return isize::MIN,
                Some(process) => match process.get_exit_queue() {
                    Some(queue) => queue,
                    None => return isize::MIN,
                },
            },
            None => panic!("Current session does not exist!"),
        },
    };
    unsafe { critical::leave_local(critical_state) };

    yield_thread(Some(exit_queue), None);

    current_thread.get_queue_data().unwrap()
}

pub fn exit_thread(exit_status: isize, critical_state: Option<bool>) -> ! {
    unsafe {
        let critical_state = match critical_state {
            Some(state) => state,
            None => crate::critical::enter_local(),
        };
        let current_thread = get_current_thread();

        current_thread.pre_exit(exit_status);
        current_thread.process().unwrap().pre_exit(exit_status);

        current_thread.clear_queue(false);
        yield_thread(None, Some(critical_state));
        panic!("Returned to thread after exit!");
    }
}

pub fn exit_process(exit_status: isize) -> ! {
    unsafe {
        let critical_state = crate::critical::enter_local();
        let current_thread = get_current_thread();
        let current_process = current_thread.process().unwrap();

        current_process.kill_threads(current_thread.id());

        exit_thread(exit_status, Some(critical_state));
    }
}

#[allow(dead_code)]
pub fn kill_thread(tid: isize) {
    unsafe {
        let critical_state = crate::critical::enter_local();
        let current_thread = get_current_thread();
        let current_process = current_thread.process().unwrap();

        match current_process.get_thread(tid) {
            Some(thread) => {
                if thread == current_thread {
                    exit_thread(128, Some(critical_state));
                } else {
                    current_process.remove_thread(tid);
                }
            }
            None => {}
        }
        crate::critical::leave_local(critical_state);
    }
}

pub fn kill_process(pid: isize) {
    let current_process = get_current_thread().process().unwrap();
    let session_lock = match current_process.session_id() {
        Some(session) => match get_session(session) {
            Some(session) => session,
            None => panic!("Current session does not exist!"),
        },
        None => return daemon::kill_process(pid),
    };
    let mut session = session_lock.lock();

    unsafe {
        let critical_state = crate::critical::enter_local();

        let remove = match session.get_process(pid) {
            Some(process) => {
                if process == current_process {
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

        crate::critical::leave_local(critical_state);
    }
}

pub fn get_current_thread() -> ThreadReference {
    get_current_thread_option().expect("No current thread when one required!")
}

pub fn get_current_thread_option() -> Option<ThreadReference> {
    THREAD_CONTROL.lock().get_current_thread()
}

pub fn preempt() {
    if !THREAD_CONTROL.lock().is_next_thread() {
        return;
    }

    if get_current_thread_option().is_none() {
        return;
    }

    queue_and_yield(Some(true));
}

pub fn handle_signals() {
    match get_current_thread_option() {
        Some(thread) => match thread.process().unwrap().handle_signals() {
            Some(val) => exit_process(val),
            None => {}
        },
        None => {}
    }
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
        current_process: &ProcessReference,
        new_process: &ProcessReference,
    ) -> error::Result<()> {
        self.stdout.copy_descriptor(current_process, new_process)?;
        self.stderr.copy_descriptor(current_process, new_process)?;
        self.stdin.copy_descriptor(current_process, new_process)
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

    pub fn copy_descriptor(
        &mut self,
        current_process: &ProcessReference,
        new_process: &ProcessReference,
    ) -> error::Result<()> {
        match self {
            StandardIOType::None | &mut StandardIOType::Console => Ok(()),
            StandardIOType::File(fd) => {
                let current_process_lock = current_process.upgrade().unwrap();
                let current_process_inner = current_process_lock.lock();
                let descriptor = current_process_inner.get_file(*fd)?;
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
