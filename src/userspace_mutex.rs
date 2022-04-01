use crate::process::{self, ThreadQueue};
use core::sync::atomic::{AtomicBool, Ordering};

pub struct UserspaceMutex {
    lock: AtomicBool,
    queue: ThreadQueue,
}

impl UserspaceMutex {
    pub const fn new() -> Self {
        UserspaceMutex {
            lock: AtomicBool::new(false),
            queue: ThreadQueue::new(),
        }
    }

    pub fn lock(&self) {
        if self
            .lock
            .compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed)
            .is_err()
        {
            process::yield_thread(Some(self.queue.into_current_queue()), None);
        }
    }

    pub fn try_lock(&self) -> bool {
        self.lock
            .compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed)
            .is_ok()
    }

    pub fn unlock(&self) {
        match self.queue.pop() {
            None => self.lock.store(false, Ordering::Relaxed),
            Some(next_thread) => {
                self.lock.store(true, Ordering::Relaxed);
                process::queue_thread(next_thread)
            }
        }
    }
}
