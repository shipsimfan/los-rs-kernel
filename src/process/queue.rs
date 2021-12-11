use super::Thread;
use crate::queue::Queue;
use core::ffi::c_void;

pub struct ThreadQueue(Queue<*mut Thread>);

unsafe fn remove(queue: *mut c_void, thread: *mut Thread) {
    let queue = &mut *(queue as *mut ThreadQueue);
    queue.remove(&mut *thread);
}

impl ThreadQueue {
    pub const fn new() -> Self {
        ThreadQueue(Queue::new())
    }

    pub unsafe fn push(&mut self, thread: &mut Thread) {
        thread.set_queue(remove, self as *mut _ as *mut _);
        self.0.push(thread);
    }

    pub unsafe fn pop_mut(&mut self) -> Option<&mut Thread> {
        match self.0.pop() {
            Some(thread) => {
                (*thread).clear_queue(true);
                Some(&mut *(thread))
            }
            None => None,
        }
    }

    pub fn is_front(&self) -> bool {
        self.0.is_front()
    }

    // Called from thread on dropping
    pub unsafe fn remove(&mut self, thread: &mut Thread) {
        self.0.remove(thread);
    }
}
