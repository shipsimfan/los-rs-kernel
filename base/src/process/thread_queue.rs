use super::Thread;
use crate::{CriticalLock, Queue};
use alloc::sync::Arc;

pub struct ThreadQueue {
    inner: Arc<CriticalLock<Queue<Thread>>>,
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
}
