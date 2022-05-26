use core::sync::atomic::{AtomicBool, Ordering};
use process::{ProcessTypes, ThreadQueue};

pub struct Mutex<T: ProcessTypes + 'static> {
    lock: AtomicBool,
    queue: ThreadQueue<T>,
}

impl<T: ProcessTypes> Mutex<T> {
    pub const fn new() -> Self {
        Mutex {
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
            process::yield_thread(Some(self.queue.current_queue()));
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
