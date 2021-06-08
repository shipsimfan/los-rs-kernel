use super::{ThreadBox, ThreadQueue};

pub struct ThreadControl {
    running_queue: ThreadQueue,
    current_thread: Option<ThreadBox>,
}

impl ThreadControl {
    pub const fn new() -> Self {
        ThreadControl {
            running_queue: ThreadQueue::new(),
            current_thread: None,
        }
    }

    pub fn queue_execution(&mut self, thread: ThreadBox) {
        self.running_queue.push(thread);
    }

    pub fn set_current_thread(&mut self, new_thread: &ThreadBox) {
        self.current_thread = Some(new_thread.clone());
    }

    pub fn get_next_thread(&mut self) -> Option<ThreadBox> {
        self.running_queue.pop()
    }

    pub fn get_current_thread(&self) -> Option<ThreadBox> {
        self.current_thread.clone()
    }

    pub fn is_next_thread(&self) -> bool {
        self.running_queue.0.len() != 0
    }
}
