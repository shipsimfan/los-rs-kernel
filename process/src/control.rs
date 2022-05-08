use crate::{process::Signals, thread_queue::ThreadQueue, CurrentQueue, ProcessOwner, Thread};
use base::{critical::CriticalLock, multi_owner::Owner};

pub struct ThreadControl<O: ProcessOwner<D, S> + 'static, D: 'static, S: Signals + 'static> {
    running_queue: ThreadQueue<O, D, S>,
    staged_thread: Option<(Owner<Thread<O, D, S>>, Option<CurrentQueue<O, D, S>>)>, // TODO: Move this into a per-core structure
    current_thread: Option<Owner<Thread<O, D, S>>>,
    daemon_owner: Owner<O>,
}

impl<O: ProcessOwner<D, S>, D, S: Signals> ThreadControl<O, D, S> {
    pub fn new() -> CriticalLock<ThreadControl<O, D, S>> {
        CriticalLock::new(ThreadControl {
            running_queue: ThreadQueue::new(),
            staged_thread: None,
            current_thread: None,
            daemon_owner: Owner::new(O::new_daemon()),
        })
    }

    pub fn daemon_owner(&self) -> Owner<O> {
        self.daemon_owner.clone()
    }

    pub fn queue_execution(&self, thread: Owner<Thread<O, D, S>>) {
        self.running_queue.push(thread);
    }

    pub fn current_thread(&self) -> &Option<Owner<Thread<O, D, S>>> {
        &self.current_thread
    }

    pub fn next_thread(&self) -> Option<Owner<Thread<O, D, S>>> {
        self.running_queue.pop()
    }

    pub fn running_queue(&self) -> CurrentQueue<O, D, S> {
        self.running_queue.current_queue()
    }

    pub fn set_staged_thread(
        &mut self,
        thread: Owner<Thread<O, D, S>>,
        queue: Option<CurrentQueue<O, D, S>>,
    ) {
        self.staged_thread = Some((thread, queue));
    }

    pub fn switch_staged_thread(
        &mut self,
    ) -> (
        Option<Owner<Thread<O, D, S>>>,
        Option<CurrentQueue<O, D, S>>,
    ) {
        let (next_thread, queue) = self.staged_thread.take().unwrap();

        let old_thread = self.current_thread.take();
        self.current_thread = Some(next_thread);

        (old_thread, queue)
    }
}
