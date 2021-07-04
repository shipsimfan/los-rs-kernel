#![allow(dead_code)]

mod control;
mod process;
mod queue;
mod thread;

use crate::map::Mappable;

pub type Thread = thread::Thread;
pub type Process = process::Process;

pub type ThreadQueue = queue::ThreadQueue;

pub type ThreadFunc = fn() -> usize;
pub type ThreadFuncContext = fn(context: usize) -> usize;

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
extern "C" fn switch_thread(next_thread: *mut Thread) {
    match unsafe { THREAD_CONTROL.get_current_thread_mut() } {
        None => {}
        Some(current_thread) => {
            if !current_thread.in_queue() {
                let current_process = current_thread.get_process_mut();
                if current_process.remove_thread(current_thread.id()) {
                    let current_process_id = current_process.id();
                    match current_process.get_session_mut() {
                        None => {} // TODO: Remove from daemon
                        Some(session) => session.remove_process(current_process_id),
                    }
                }
            }
        }
    }

    unsafe { THREAD_CONTROL.set_current_thread(next_thread) };
}

pub fn create_thread(entry: ThreadFunc) -> usize {
    let current_thread = get_current_thread_mut();
    let current_process = current_thread.get_process_mut();
    current_process.create_thread(entry as usize, 0)
}

pub fn create_process(entry: ThreadFunc) -> usize {
    let current_thread = get_current_thread_mut();
    let current_process = current_thread.get_process_mut();
    match current_process.get_session_mut() {
        None => panic!("Creating daemon process!"),
        Some(current_session) => current_session.create_process(entry as usize, 0),
    }
}

pub fn queue_thread(thread: &mut Thread) {
    unsafe {
        let return_interrupts = get_rflags() & (1 << 9) == 0;
        asm!("cli");
        THREAD_CONTROL.queue_execution(thread);
        if return_interrupts {
            asm!("sti");
        }
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

                                if current_thread.compare_process(next_thread) {
                                    next_thread.get_process().set_address_space_as_current();
                                }

                                current_thread.get_stack_pointer_location()
                            }
                        },
                        {
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

pub fn wait_thread(tid: usize) -> usize {
    let current_thread = get_current_thread_mut();
    match current_thread.get_process_mut().get_thread_mut(tid) {
        None => return usize::MAX,
        Some(thread) => thread.insert_into_exit_queue(current_thread),
    }

    yield_thread();

    current_thread.get_queue_data()
}

pub fn wait_process(pid: usize) -> usize {
    let current_thread = get_current_thread_mut();
    match current_thread.get_process_mut().get_session_mut() {
        None => panic!("Waiting on a daemon!"),
        Some(session) => match session.get_process_mut(pid) {
            None => return usize::MAX,
            Some(process) => process.insert_into_exit_queue(current_thread),
        },
    }

    yield_thread();

    current_thread.get_queue_data()
}

pub fn exit_thread(exit_status: usize) {
    let current_thread = get_current_thread_mut();

    current_thread.pre_exit(exit_status);
    current_thread.get_process_mut().pre_exit(exit_status);

    current_thread.clear_queue();
    yield_thread();
}

pub fn get_current_thread_option() -> Option<&'static Thread> {
    unsafe {
        let return_interrupts = get_rflags() & (1 << 9) == 0;
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
        let return_interrupts = get_rflags() & (1 << 9) == 0;
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
