use super::{thread::ThreadOwner, CurrentQueue, ThreadQueue, ThreadReference};

pub struct ThreadControl {
    running_queue: ThreadQueue,
    current_thread: Option<ThreadOwner>,
}

impl ThreadControl {
    pub const fn new() -> Self {
        ThreadControl {
            running_queue: ThreadQueue::new(),
            current_thread: None,
        }
    }

    pub fn get_current_thread(&self) -> Option<ThreadReference> {
        self.current_thread
            .as_ref()
            .map(|thread| thread.reference())
    }

    // Returns a thread owner if the thread needs to be dropped
    // The dropping must take place outside of the lock for thread control
    pub fn set_current_thread(
        &mut self,
        new_thread: ThreadOwner,
        new_queue: Option<CurrentQueue>,
    ) -> Option<ThreadOwner> {
        let current_thread = match self.current_thread.take() {
            Some(thread) => thread,
            None => {
                self.current_thread = Some(new_thread);
                return None;
            }
        };

        let ret = match new_queue {
            Some(queue) => unsafe {
                current_thread.set_queue(queue.clone());
                queue.add(current_thread);
                None
            },
            None => Some(current_thread),
        };

        self.current_thread = Some(new_thread);

        ret
    }

    pub unsafe fn get_next_thread(&mut self) -> Option<ThreadOwner> {
        self.running_queue.pop()
    }

    pub fn is_next_thread(&self) -> bool {
        self.running_queue.is_front()
    }

    pub fn queue_execution(&self, thread: ThreadOwner) {
        self.running_queue.push(thread)
    }

    pub fn get_current_queue(&self) -> CurrentQueue {
        self.running_queue.into_current_queue()
    }
}
