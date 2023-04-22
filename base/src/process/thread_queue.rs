use super::Thread;
use crate::{CriticalKey, CriticalLock, Queue};
use alloc::sync::Arc;

#[derive(Clone)]
pub struct ThreadQueue {
    inner: Arc<CriticalLock<Queue<Thread>>>,
}

pub struct ThreadQueueGuard {
    queue: Arc<CriticalLock<Queue<Thread>>>,
    queue_ref: &'static mut Queue<Thread>,
    key: Option<CriticalKey>,
}

impl ThreadQueue {
    pub fn new() -> Self {
        ThreadQueue {
            inner: Arc::new(CriticalLock::new(Queue::new())),
        }
    }

    pub fn push(&self, thread: Thread) {
        if thread.is_killed() {
            return;
        }

        self.inner.lock().push(thread);
    }

    pub fn pop(&self) -> Option<Thread> {
        let mut queue = self.inner.lock();

        while let Some(thread) = queue.pop() {
            if !thread.is_killed() {
                return Some(thread);
            }
        }

        None
    }

    pub fn pop_all<F>(&self, f: F)
    where
        F: Fn(Thread),
    {
        let mut queue = self.inner.lock();

        while let Some(thread) = queue.pop() {
            f(thread);
        }
    }

    pub fn lock(self) -> ThreadQueueGuard {
        let (queue_ref, key) = unsafe { self.inner.lock_no_guard(true) };

        ThreadQueueGuard {
            queue: self.inner,
            queue_ref,
            key,
        }
    }
}

impl ThreadQueueGuard {
    pub fn push(self, thread: Thread) {
        self.queue_ref.push(thread);
    }
}

impl Drop for ThreadQueueGuard {
    fn drop(&mut self) {
        unsafe { self.queue.unlock(self.key.take()) };
    }
}
