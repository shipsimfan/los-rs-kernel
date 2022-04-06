use crate::process::{self, ThreadQueue};

pub struct ConditionalVariable {
    queue: ThreadQueue,
}

impl ConditionalVariable {
    pub const fn new() -> Self {
        ConditionalVariable {
            queue: ThreadQueue::new(),
        }
    }

    pub fn wait(&self) {
        process::yield_thread(Some(self.queue.into_current_queue()), None);
    }

    pub fn signal(&self) {
        match self.queue.pop() {
            None => {},
            Some(next_thread) => {
                process::queue_thread(next_thread);
            }
        }
    }

    pub fn broadcast(&self) {
        while let Some(next_thread) = self.queue.pop() {
            process::queue_thread(next_thread);
        }
    }
}