use super::{thread::FloatingPointStorage, Thread, ThreadQueueGuard};
use crate::{CriticalKey, LocalState, ProcessManager};

pub(crate) struct LocalProcessController {
    current_thread: Option<Thread>,

    // Switch information
    key: Option<CriticalKey>,
    next_thread: Option<Thread>,
    target_queue: Option<ThreadQueueGuard>,

    // Null thread info
    null_floating_point_storage: FloatingPointStorage,
    null_stack_top: usize,
    null_stack_pointer: usize,
}

impl LocalProcessController {
    pub(crate) fn new(null_stack_top: usize) -> Self {
        LocalProcessController {
            current_thread: None,

            next_thread: None,
            target_queue: None,

            key: None,

            null_floating_point_storage: FloatingPointStorage::new(),
            null_stack_top,
            null_stack_pointer: 0, // First access will be a save when this processor yields to an actual thread
        }
    }

    pub fn current_thread(&self) -> &Thread {
        self.current_thread_opt().unwrap()
    }

    pub fn current_thread_opt(&self) -> Option<&Thread> {
        self.current_thread.as_ref()
    }

    pub(super) unsafe fn load_float(&self) {
        self.null_floating_point_storage.load_float();
    }

    pub(super) unsafe fn save_float(&self) {
        self.null_floating_point_storage.save_float();
    }

    pub(super) unsafe fn set_interrupt_stack(&self) {
        LocalState::get()
            .gdt()
            .set_interrupt_stack(self.null_stack_top as u64);
    }

    pub(super) unsafe fn null_stack_pointer_location(&self) -> *mut usize {
        &self.null_stack_pointer as *const usize as *mut usize
    }

    pub(super) unsafe fn set_target_queue(&mut self, target_queue: Option<ThreadQueueGuard>) {
        self.target_queue = target_queue;
    }

    pub(super) unsafe fn set_next_thread(&mut self, next_thread: Option<Thread>) {
        self.next_thread = next_thread;
    }

    pub(super) unsafe fn set_key(&mut self, key: CriticalKey) {
        assert!(self.key.is_none());
        self.key = Some(key);
    }

    pub(super) unsafe fn take_key(&mut self) -> CriticalKey {
        self.key.take().unwrap()
    }

    pub(super) unsafe fn set_next_thread_and_queue_old(&mut self) {
        let old_thread = self.current_thread.take();
        self.current_thread = self.next_thread.take();

        match self.target_queue.take() {
            Some(queue) => match old_thread {
                Some(thread) => match thread.is_killed() {
                    true => {
                        drop(queue);
                        drop(thread);
                    }
                    false => queue.push(thread),
                },
                None => panic!("Cannot push a null thread into a queue"),
            },
            None => match old_thread {
                Some(thread) => ProcessManager::get().queue_thread(thread),
                None => {}
            },
        }
    }
}
