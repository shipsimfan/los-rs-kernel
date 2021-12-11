use super::{Thread, ThreadQueue};

pub struct ThreadControl {
    running_queue: ThreadQueue,
    current_thread: Option<*mut Thread>,
}

impl ThreadControl {
    pub const fn new() -> Self {
        ThreadControl {
            running_queue: ThreadQueue::new(),
            current_thread: None,
        }
    }

    pub fn get_current_thread(&self) -> Option<&'static Thread> {
        match self.current_thread {
            Some(thread) => Some(unsafe { &*thread }),
            None => None,
        }
    }

    pub fn get_current_thread_mut(&self) -> Option<&'static mut Thread> {
        match self.current_thread {
            Some(thread) => Some(unsafe { &mut *thread }),
            None => None,
        }
    }

    pub fn set_current_thread(&mut self, new_thread: *mut Thread) {
        self.current_thread = Some(new_thread);
    }

    pub unsafe fn get_next_thread(&mut self) -> Option<&mut Thread> {
        self.running_queue.pop_mut()
    }

    pub fn is_next_thread(&self) -> bool {
        self.running_queue.is_front()
    }

    pub unsafe fn queue_execution(&mut self, thread: &mut Thread) {
        self.running_queue.push(thread);
    }
}
