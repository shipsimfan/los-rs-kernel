use crate::{thread_queue::ThreadQueue, CurrentQueue, ProcessTypes, Thread};
use base::{critical::CriticalLock, multi_owner::Owner};

pub struct ThreadControl<T: ProcessTypes + 'static> {
    running_queue: ThreadQueue<T>,
    staged_thread: Option<(Owner<Thread<T>>, Option<CurrentQueue<T>>, bool)>, // TODO: Move this into a per-core structure
    current_thread: Option<Owner<Thread<T>>>,
    daemon_owner: Owner<T::Owner>,
}

impl<T: ProcessTypes> ThreadControl<T> {
    pub fn new() -> CriticalLock<ThreadControl<T>> {
        CriticalLock::new(ThreadControl {
            running_queue: ThreadQueue::new(),
            staged_thread: None,
            current_thread: None,
            daemon_owner: Owner::new(T::new_daemon()),
        })
    }

    pub fn daemon_owner(&self) -> &Owner<T::Owner> {
        &self.daemon_owner
    }

    pub fn queue_execution(&self, thread: Owner<Thread<T>>) {
        self.running_queue.push(thread);
    }

    pub fn current_thread(&self) -> &Option<Owner<Thread<T>>> {
        &self.current_thread
    }

    pub fn next_thread(&self) -> Option<Owner<Thread<T>>> {
        self.running_queue.pop()
    }

    pub fn is_next_thread(&self) -> bool {
        self.running_queue.length() != 0
    }

    pub fn running_queue(&self) -> CurrentQueue<T> {
        self.running_queue.current_queue()
    }

    pub fn set_staged_thread(
        &mut self,
        thread: Owner<Thread<T>>,
        queue: Option<CurrentQueue<T>>,
        kill: bool,
    ) {
        self.staged_thread = Some((thread, queue, kill));
    }

    pub fn switch_staged_thread(
        &mut self,
    ) -> (Option<Owner<Thread<T>>>, Option<CurrentQueue<T>>, bool) {
        let (next_thread, queue, kill) = self.staged_thread.take().unwrap();

        let old_thread = self.current_thread.take();
        self.current_thread = Some(next_thread);

        (old_thread, queue, kill)
    }
}
