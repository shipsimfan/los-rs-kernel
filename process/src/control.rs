use crate::{process::Signals, thread_queue::ThreadQueue, ProcessOwner, Thread};
use base::multi_owner::{Owner, Reference};

pub struct ThreadControl<O: ProcessOwner<D, S> + 'static, D: 'static, S: Signals + 'static> {
    running_queue: ThreadQueue<O, D, S>,
    current_thread: Option<Owner<Thread<O, D, S>>>,
    daemon_owner: Owner<O>,
}

impl<O: ProcessOwner<D, S>, D, S: Signals> ThreadControl<O, D, S> {
    pub const fn new(daemon_owner: Owner<O>) -> ThreadControl<O, D, S> {
        ThreadControl {
            running_queue: ThreadQueue::new(),
            current_thread: None,
            daemon_owner,
        }
    }

    pub fn daemon_owner(&self) -> Owner<O> {
        self.daemon_owner.clone()
    }

    pub fn queue_execution(&self, thread: Owner<Thread<O, D, S>>) {
        self.running_queue.push(thread);
    }

    pub fn get_current_thread(&self) -> Option<Reference<Thread<O, D, S>>> {
        self.current_thread.as_ref().map(|thread| thread.as_ref())
    }
}
