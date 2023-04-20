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
        loop {
            match self.inner.lock().pop() {
                Some(thread) => match thread.is_killed() {
                    true => continue,
                    false => return Some(thread),
                },
                None => return None,
            }
        }
    }
}
