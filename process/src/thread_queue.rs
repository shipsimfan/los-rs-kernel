use crate::{thread::QueueAccess, CurrentQueue, ProcessTypes, Thread};
use alloc::boxed::Box;
use base::{
    critical::CriticalLock,
    multi_owner::{Owner, Reference},
    queue::Queue,
};

#[allow(unused)]
enum QueuedThread<T: ProcessTypes + 'static> {
    Actual(Owner<Thread<T>>),
    Compare(Reference<Thread<T>>),
}

pub struct ThreadQueue<T: ProcessTypes + 'static>(CriticalLock<Queue<QueuedThread<T>>>);

struct ThreadQueueAccess;

impl<T: ProcessTypes> ThreadQueue<T> {
    pub const fn new() -> Self {
        ThreadQueue(CriticalLock::new(Queue::new()))
    }

    pub fn push(&self, thread: Owner<Thread<T>>) {
        thread.lock(|thread| thread.set_current_queue(self.current_queue()));
        self.0.lock().push(QueuedThread::Actual(thread));
    }

    pub fn pop(&self) -> Option<Owner<Thread<T>>> {
        self.0.lock().pop().map(|t| {
            let t = t.unwrap();
            t.lock(|t| unsafe { t.clear_queue(true) });
            t
        })
    }

    pub fn remove(&self, thread: &Reference<Thread<T>>) -> Option<Owner<Thread<T>>> {
        self.0
            .lock()
            .remove(QueuedThread::Compare(thread.clone()))
            .map(|thread| thread.into())
    }

    pub fn current_queue(&self) -> CurrentQueue<T> {
        CurrentQueue::new(Box::new(ThreadQueueAccess), self as *const _ as *mut _)
    }
}

impl<T: ProcessTypes> QueuedThread<T> {
    pub fn unwrap(self) -> Owner<Thread<T>> {
        match self {
            QueuedThread::Actual(thread) => thread,
            QueuedThread::Compare(_) => {
                panic!("\"Compare\" queued thread should never actually be in the queue!")
            }
        }
    }
}

impl<T: ProcessTypes> QueueAccess<T> for ThreadQueueAccess {
    unsafe fn add(&self, queue: *mut core::ffi::c_void, thread: Owner<Thread<T>>) {
        let queue = &mut *(queue as *mut ThreadQueue<T>);
        queue.push(thread)
    }

    unsafe fn remove(
        &self,
        queue: *mut core::ffi::c_void,
        thread: &Reference<Thread<T>>,
    ) -> Option<Owner<Thread<T>>> {
        let queue = &mut *(queue as *mut ThreadQueue<T>);
        queue.remove(thread)
    }
}

impl<T: ProcessTypes> PartialEq for QueuedThread<T> {
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

impl<T: ProcessTypes> Into<Owner<Thread<T>>> for QueuedThread<T> {
    fn into(self) -> Owner<Thread<T>> {
        match self {
            QueuedThread::Actual(thread) => thread,
            QueuedThread::Compare(_) => {
                panic!("Comparing queued thread cannot be turned into thread owner")
            }
        }
    }
}
