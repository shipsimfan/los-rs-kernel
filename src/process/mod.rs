#![allow(dead_code)]

mod control;
mod loader;
mod process;
mod queue;
mod thread;

use alloc::{string::String, vec::Vec};

use crate::{
    filesystem::{self, open_directory, DirectoryDescriptor},
    map::Mappable,
    memory::KERNEL_VMA,
};

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
}

const USERSPACE_CONTEXT_LOCATION: *const UserspaceContext =
    0x700000000000 as *const UserspaceContext;

static mut THREAD_CONTROL: control::ThreadControl = control::ThreadControl::new();

extern "C" {
    #[allow(improper_ctypes)]
    fn perform_yield(
        save_location: *const usize,
        load_location: *const usize,
        next_thread: *mut Thread,
    );
    fn get_rflags() -> usize;
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
                        None => {} // TODO: Remove from daemon
                        Some(session) => (*session.as_ptr()).remove_process(current_process_id),
                    }
                }
            }
        }
    }

    THREAD_CONTROL.set_current_thread(next_thread);
}

pub fn execute(
    filepath: &str,
    args: Vec<String>,
    environment: Vec<String>,
) -> Result<isize, crate::error::Status> {
    // Load the executable
    let buffer = filesystem::read(filepath)?;

    // Parse the elf header
    let entry = loader::verify_executable(&buffer)?;
    if entry >= KERNEL_VMA {
        return Err(crate::error::Status::InvalidArgument);
    }

    // Figure out working directory
    let working_directory = match get_current_thread_mut()
        .get_process_mut()
        .get_current_working_directory()
    {
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
    let pid = do_create_process(
        entry,
        USERSPACE_CONTEXT_LOCATION as usize,
        Some(working_directory),
    );

    // Lock session
    let current_process = get_current_thread_mut().get_process_mut();
    let session_lock = current_process.get_session_mut().unwrap();
    let mut session = session_lock.lock();

    unsafe { asm!("cli") };
    let new_process = session.get_process_mut(pid).unwrap();

    // Switch to new process address space
    new_process.set_address_space_as_current();

    // Copy the executable into the address space
    loader::load_executable(&buffer)?;

    // Copy arguments and environment variables into the address space
    unsafe {
        let context_location = USERSPACE_CONTEXT_LOCATION as usize;
        let arg_list_start = context_location + core::mem::size_of::<UserspaceContext>();
        let env_list_start = arg_list_start + (core::mem::size_of::<*mut u8>() * (args.len() + 1));
        let args_start =
            env_list_start + (core::mem::size_of::<*mut u8>() * (environment.len() + 1));
        let mut arg_list = arg_list_start as *mut *mut u8;
        let mut env_list = env_list_start as *mut *mut u8;

        let context = context_location as *mut UserspaceContext;
        *context = UserspaceContext {
            argc: args.len(),
            argv: arg_list as *const *const u8,
            envp: env_list as *const *const u8,
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
    }

    // Return to current process address space
    current_process.set_address_space_as_current();

    // Return the process id
    unsafe { asm!("sti") };
    Ok(pid)
}

pub fn create_thread(entry: ThreadFunc) -> isize {
    create_thread_raw(entry as usize)
}

pub fn create_thread_raw(entry: usize) -> isize {
    let current_thread = get_current_thread_mut();
    let current_process = current_thread.get_process_mut();
    current_process.create_thread(entry, 0)
}

fn do_create_process(
    entry: usize,
    context: usize,
    working_directory: Option<DirectoryDescriptor>,
) -> isize {
    let current_thread = get_current_thread_mut();
    let current_process = current_thread.get_process_mut();
    match current_process.get_session_mut() {
        None => panic!("Creating daemon process!"),
        Some(current_session) => current_session.lock().create_process(
            entry,
            context,
            working_directory,
            current_session.clone(),
        ),
    }
}

pub fn create_process(entry: ThreadFunc, working_directory: Option<DirectoryDescriptor>) -> isize {
    do_create_process(entry as usize, 0, working_directory)
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
        Some(thread) => thread.insert_into_exit_queue(current_thread),
    }

    yield_thread();

    current_thread.get_queue_data()
}

pub fn wait_process(pid: isize) -> isize {
    let current_thread = get_current_thread_mut();
    match current_thread.get_process_mut().get_session_mut() {
        None => panic!("Waiting on a daemon!"),
        Some(session) => match session.lock().get_process_mut(pid) {
            None => return isize::MIN,
            Some(process) => process.insert_into_exit_queue(current_thread),
        },
    }

    yield_thread();

    current_thread.get_queue_data()
}

pub fn exit_thread(exit_status: isize) -> ! {
    let current_thread = get_current_thread_mut();

    current_thread.pre_exit(exit_status);
    current_thread.get_process_mut().pre_exit(exit_status);

    current_thread.clear_queue();
    yield_thread();
    panic!("Returned to thread after exit!");
}

pub fn get_current_thread() -> &'static Thread {
    get_current_thread_option().expect("No current thread when one required!")
}

pub fn get_current_thread_option() -> Option<&'static Thread> {
    unsafe {
        let return_interrupts = get_rflags() & (1 << 9) != 0;
        asm!("cli");
        let ret = THREAD_CONTROL.get_current_thread();
        if return_interrupts {
            asm!("sti");
        }
        ret
    }
}

pub fn get_current_thread_mut() -> &'static mut Thread {
    get_current_thread_mut_option().expect("No current thread when one required!")
}

pub fn get_current_thread_mut_option() -> Option<&'static mut Thread> {
    unsafe {
        let return_interrupts = get_rflags() & (1 << 9) != 0;
        asm!("cli");
        let ret = THREAD_CONTROL.get_current_thread_mut();
        if return_interrupts {
            asm!("sti");
        }
        ret
    }
}

pub fn preempt() {
    unsafe {
        if !THREAD_CONTROL.is_next_thread() {
            return;
        }

        if get_current_thread_option().is_none() {
            return;
        }
    }

    queue_and_yield();
}
