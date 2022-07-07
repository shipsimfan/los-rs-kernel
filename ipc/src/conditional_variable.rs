use process::{ProcessTypes, ThreadQueue};

pub struct ConditionalVariable<T: ProcessTypes + 'static> {
    queue: ThreadQueue<T>,
}

impl<T: ProcessTypes> ConditionalVariable<T> {
    pub fn new() -> Self {
        ConditionalVariable {
            queue: ThreadQueue::new(),
        }
    }

    pub fn wait(&self) {
        process::yield_thread(Some(self.queue.current_queue()));
    }

    pub fn signal(&self) {
        match self.queue.pop() {
            None => {}
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
