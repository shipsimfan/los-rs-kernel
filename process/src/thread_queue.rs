use crate::{process::Signals, ProcessOwner, Thread};
use base::{critical::CriticalLock, multi_owner::Owner, queue::Queue};

#[allow(unused)]
enum QueuedThread<O: ProcessOwner<D, S> + 'static, D: 'static, S: Signals + 'static> {
    Actual(Owner<Thread<O, D, S>>),
    Compare(*const Thread<O, D, S>),
}

pub struct ThreadQueue<O: ProcessOwner<D, S> + 'static, D: 'static, S: Signals + 'static>(
    CriticalLock<Queue<QueuedThread<O, D, S>>>,
);

impl<O: ProcessOwner<D, S>, D, S: Signals> ThreadQueue<O, D, S> {
    pub const fn new() -> Self {
        ThreadQueue(CriticalLock::new(Queue::new()))
    }

    pub fn push(&self, thread: Owner<Thread<O, D, S>>) {
        // TODO: insert self as current queue
        self.0.lock().push(QueuedThread::Actual(thread));
    }

    pub fn pop(&self) -> Option<Owner<Thread<O, D, S>>> {
        self.0.lock().pop().map(|t| {
            let t = t.unwrap();
            t.lock(|t| unsafe { t.clear_queue(true) });
            t
        })
    }
}

impl<O: ProcessOwner<D, S>, D, S: Signals> QueuedThread<O, D, S> {
    pub fn unwrap(self) -> Owner<Thread<O, D, S>> {
        match self {
            QueuedThread::Actual(thread) => thread,
            QueuedThread::Compare(_) => {
                panic!("\"Compare\" queued thread should never actually be in the queue!")
            }
        }
    }
}
