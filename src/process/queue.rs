use super::Thread;
use crate::queue::Queue;

pub struct ThreadQueue(Queue<*mut Thread>);

impl ThreadQueue {
    pub const fn new() -> Self {
        ThreadQueue(Queue::new())
    }

    pub fn push(&mut self, thread: &mut Thread) {
        thread.set_queue(self);
        self.0.push(thread);
    }

    pub fn pop(&mut self) -> Option<&Thread> {
        match self.pop_mut() {
            Some(thread) => Some(thread),
            None => None,
        }
    }

    pub fn pop_mut(&mut self) -> Option<&mut Thread> {
        match self.0.pop() {
            Some(thread) => unsafe {
                (*thread).clear_queue();
                Some(&mut *(thread))
            },
            None => None,
        }
    }

    pub fn is_front(&self) -> bool {
        self.0.is_front()
    }

    // Called from thread on dropping
    pub fn remove(&mut self, thread: &mut Thread) {
        self.0.remove(thread);
    }
}

/*
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
} */
