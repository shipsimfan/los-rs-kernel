mod control;
mod process;
mod thread;

use crate::{locks::Mutex, queue::Queue, session::SessionBox};
use alloc::{
    sync::{Arc, Weak},
    vec::Vec,
};

struct Daemon {
    pub processes: Vec<WeakProcessBox>,
    pub next_id: process::PID,
}

pub struct ThreadQueue(Queue<ThreadBox>);

type Lock<T> = Mutex<T>;

pub type PID = process::PID;
pub type TID = thread::TID;
pub type Process = process::Process;
pub type Thread = thread::Thread;
pub type ProcessBox = Arc<Lock<Process>>;
pub type ThreadBox = Arc<Lock<Thread>>;
pub type WeakProcessBox = Weak<Lock<Process>>;
pub type WeakThreadBox = Weak<Lock<Thread>>;
pub type ThreadFunc = fn();
pub type ThreadFuncContext = fn(context: usize);

static mut THREAD_CONTROL: control::ThreadControl = control::ThreadControl::new();

static DAEMON: Lock<Daemon> = Lock::new(Daemon {
    processes: Vec::new(),
    next_id: 0,
});

extern "C" {
    fn perform_yield(save_location: *const usize, load_location: *const usize);
    fn get_rflags() -> usize;
}

pub fn create_process(entry: ThreadFunc, session: Option<&SessionBox>) {
    match session {
        None => {
            let mut daemon = DAEMON.lock();
            let new_process = Arc::new(Lock::new(process::Process::new(daemon.next_id, None)));
            new_process
                .lock()
                .create_thread(&new_process, entry as usize, 0);
            daemon.processes.push(Arc::downgrade(&new_process));
            daemon.next_id += 1;
        }
        Some(session) => session
            .lock()
            .create_process(entry as usize, 0, Some(session)),
    }
}

pub fn _create_process_with_context(
    entry: ThreadFunc,
    context: usize,
    session: Option<&mut SessionBox>,
) {
    match session {
        None => {
            let mut daemon = DAEMON.lock();
            let new_process = Arc::new(Lock::new(process::Process::new(daemon.next_id, None)));
            new_process
                .lock()
                .create_thread(&new_process, entry as usize, context);
            daemon.processes.push(Arc::downgrade(&new_process));
            daemon.next_id += 1;
        }
        Some(session) => session
            .lock()
            .create_process(entry as usize, context, Some(session)),
    }
}

pub fn create_thread(entry: ThreadFunc) {
    let current_process = get_current_thread().lock().get_process();

    current_process
        .lock()
        .create_thread(&current_process, entry as usize, 0);
}

pub fn _create_thread_with_context(entry: ThreadFuncContext, context: usize) {
    let current_process = get_current_thread().lock().get_process();

    current_process
        .lock()
        .create_thread(&current_process, entry as usize, context);
}

pub fn queue_thread(thread: ThreadBox) {
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
    queue_thread(get_current_thread());

    yield_thread();
}

pub fn get_current_thread() -> ThreadBox {
    get_current_thread_option().unwrap()
}

pub fn get_current_thread_option() -> Option<ThreadBox> {
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

pub fn yield_thread() {
    loop {
        unsafe {
            asm!("cli");
            while let Some(thread_lock) = THREAD_CONTROL.get_next_thread() {
                let default_location: usize = 0;
                let (save_location, load_location) = {
                    let next_thread = thread_lock.lock();
                    (
                        match THREAD_CONTROL.get_current_thread() {
                            None => &default_location as *const usize,
                            Some(current_thread_lock) => {
                                let current_thread = current_thread_lock.lock();
                                current_thread.save_float();

                                if !Arc::ptr_eq(
                                    &current_thread.get_process(),
                                    &next_thread.get_process(),
                                ) {
                                    next_thread
                                        .get_process()
                                        .lock()
                                        .set_address_space_as_current();
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

                THREAD_CONTROL.set_current_thread(&thread_lock);

                perform_yield(save_location, load_location);

                return;
            }
            {
                asm!("sti; hlt")
            }
        }
    }
}

pub fn exit_thread() {
    {
        get_current_thread().lock().kill();
    }

    yield_thread();
}

pub fn preempt() {
    unsafe {
        if !THREAD_CONTROL.is_next_thread() {
            return;
        }

        let current_thread_lock = get_current_thread();
        let current_thread = current_thread_lock.try_lock();
        if current_thread.is_none() {
            drop(current_thread);
            drop(current_thread_lock);
            return;
        }

        let current_thread = current_thread.unwrap();
        let current_process = current_thread.get_process();
        if current_process.try_lock().is_none() {
            drop(current_process);
            drop(current_thread);
            drop(current_thread_lock);
            return;
        }

        drop(current_process);
        drop(current_thread);
        drop(current_thread_lock);
    }

    crate::interrupts::irq::end_interrupt();
    queue_and_yield();
}

impl ThreadQueue {
    pub const fn new() -> Self {
        ThreadQueue(Queue::new())
    }

    pub fn push(&mut self, thread: ThreadBox) {
        self.0.push(thread)
    }

    pub fn pop(&mut self) -> Option<ThreadBox> {
        while let Some(t) = self.0.pop() {
            if !t.lock().is_killed() {
                return Some(t);
            }
        }

        None
    }
}
