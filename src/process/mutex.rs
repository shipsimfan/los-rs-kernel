use super::{Thread, ThreadQueue};
use core::{
    ptr::null_mut,
    sync::atomic::{AtomicPtr, Ordering},
};

pub struct ProcessMutex {
    lock: AtomicPtr<Thread>,
    queue: ThreadQueue,
}

impl ProcessMutex {
    pub const fn new() -> Self {
        ProcessMutex {
            lock: AtomicPtr::new(null_mut()),
            queue: ThreadQueue::new(),
        }
    }

    pub fn lock(&mut self) {
        unsafe { asm!("cli") };

        let current_thread = unsafe { super::get_current_thread_mut_cli() };

        if self
            .lock
            .compare_exchange(
                null_mut(),
                current_thread,
                Ordering::Acquire,
                Ordering::Relaxed,
            )
            .is_err()
        {
            unsafe { (*(&self.queue as *const _ as *mut ThreadQueue)).push(current_thread) };
            super::yield_thread();
        }

        unsafe { asm!("sti") };
    }

    pub fn unlock(&mut self) {
        unsafe { asm!("cli") };
        match self.queue.pop_mut() {
            None => self.lock.store(null_mut(), Ordering::Relaxed),
            Some(next_thread) => {
                self.lock.store(next_thread, Ordering::Relaxed);
                unsafe { super::queue_thread_cli(next_thread) };
            }
        }
        unsafe { asm!("sti") };
    }
}
