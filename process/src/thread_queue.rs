use crate::{process::Signals, thread::QueueAccess, CurrentQueue, ProcessOwner, Thread};
use alloc::boxed::Box;
use base::{
    critical::CriticalLock,
    multi_owner::{Owner, Reference},
    queue::Queue,
};

#[allow(unused)]
enum QueuedThread<O: ProcessOwner<D, S> + 'static, D: 'static, S: Signals + 'static> {
    Actual(Owner<Thread<O, D, S>>),
    Compare(Reference<Thread<O, D, S>>),
}

pub struct ThreadQueue<O: ProcessOwner<D, S> + 'static, D: 'static, S: Signals + 'static>(
    CriticalLock<Queue<QueuedThread<O, D, S>>>,
);

struct ThreadQueueAccess;

impl<O: ProcessOwner<D, S>, D, S: Signals> ThreadQueue<O, D, S> {
    pub const fn new() -> Self {
        ThreadQueue(CriticalLock::new(Queue::new()))
    }

    pub fn push(&self, thread: Owner<Thread<O, D, S>>) {
        thread.lock(|thread| thread.set_current_queue(self.current_queue()));
        self.0.lock().push(QueuedThread::Actual(thread));
    }

    pub fn pop(&self) -> Option<Owner<Thread<O, D, S>>> {
        self.0.lock().pop().map(|t| {
            let t = t.unwrap();
            t.lock(|t| unsafe { t.clear_queue(true) });
            t
        })
    }

    pub fn remove(&self, thread: &Reference<Thread<O, D, S>>) -> Option<Owner<Thread<O, D, S>>> {
        self.0
            .lock()
            .remove(QueuedThread::Compare(thread.clone()))
            .map(|thread| thread.into())
    }

    pub fn current_queue(&self) -> CurrentQueue<O, D, S> {
        CurrentQueue::new(Box::new(ThreadQueueAccess), self as *const _ as *mut _)
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

impl<O: ProcessOwner<D, S> + 'static, D: 'static, S: Signals + 'static> QueueAccess<O, D, S>
    for ThreadQueueAccess
{
    unsafe fn add(&self, queue: *mut core::ffi::c_void, thread: Owner<Thread<O, D, S>>) {
        let queue = &mut *(queue as *mut ThreadQueue<O, D, S>);
        queue.push(thread)
    }

    unsafe fn remove(
        &self,
        queue: *mut core::ffi::c_void,
        thread: &Reference<Thread<O, D, S>>,
    ) -> Option<Owner<Thread<O, D, S>>> {
        let queue = &mut *(queue as *mut ThreadQueue<O, D, S>);
        queue.remove(thread)
    }
}

impl<O: ProcessOwner<D, S> + 'static, D: 'static, S: Signals + 'static> PartialEq
    for QueuedThread<O, D, S>
{
    fn eq(&self, other: &Self) -> bool {
        match self {
            QueuedThread::Actual(thread) => match other {
                QueuedThread::Actual(thread2) => thread.compare(thread2),
                QueuedThread::Compare(thread2) => thread.compare_ref(thread2),
            },
            QueuedThread::Compare(thread) => match other {
                QueuedThread::Actual(thread2) => thread2.compare_ref(thread),
                QueuedThread::Compare(thread2) => thread.compare(thread2),
            },
        }
    }
}

impl<O: ProcessOwner<D, S> + 'static, D: 'static, S: Signals + 'static> Into<Owner<Thread<O, D, S>>>
    for QueuedThread<O, D, S>
{
    fn into(self) -> Owner<Thread<O, D, S>> {
        match self {
            QueuedThread::Actual(thread) => thread,
            QueuedThread::Compare(_) => {
                panic!("Comparing queued thread cannot be turned into thread owner")
            }
        }
    }
}
