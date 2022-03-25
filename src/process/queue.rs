use super::{AddFn, CurrentQueue, ThreadInner, ThreadOwner};
use crate::{
    critical::CriticalLock,
    queue::{Queue, SortedQueue},
};
use core::{ffi::c_void, sync::atomic::AtomicPtr};

enum QueuedThread {
    Actual(ThreadOwner),
    Compare(*const ThreadInner),
}

pub struct ThreadQueue(CriticalLock<Queue<QueuedThread>>);

pub struct SortedThreadQueue<I: PartialOrd + Copy>(CriticalLock<SortedQueue<I, QueuedThread>>);

unsafe fn remove(queue: *mut c_void, thread: *const ThreadInner) {
    let queue = &mut *(queue as *mut ThreadQueue);
    queue.remove(thread);
}

unsafe fn remove_sorted<I: PartialOrd + Copy>(queue: *mut c_void, thread: *const ThreadInner) {
    let queue = &mut *(queue as *mut SortedThreadQueue<I>);
    queue.remove(thread);
}

unsafe fn add(queue: *mut c_void, thread: ThreadOwner) {
    let queue = &mut *(queue as *mut ThreadQueue);
    queue.push(thread)
}

unsafe fn add_sorted<I: PartialOrd + Copy>(queue: *mut c_void, thread: ThreadOwner, value: I) {
    let queue = &mut *(queue as *mut SortedThreadQueue<I>);
    queue.insert(thread, value)
}

impl ThreadQueue {
    pub const fn new() -> Self {
        ThreadQueue(CriticalLock::new(Queue::new()))
    }

    pub fn push(&self, thread: ThreadOwner) {
        //thread.set_queue(self.into_current_queue());
        self.0.lock().push(QueuedThread::Actual(thread));
    }

    pub fn pop(&self) -> Option<ThreadOwner> {
        self.0.lock().pop().map(|queued| match queued {
            QueuedThread::Actual(thread) => thread,
            _ => panic!("\"Compare\" queued thread should never actually be in the queue!"),
        })
    }

    pub fn is_front(&self) -> bool {
        self.0.lock().is_front()
    }

    // Called from thread on dropping
    pub unsafe fn remove(&self, thread: *const ThreadInner) {
        self.0.lock().remove(QueuedThread::Compare(thread));
    }

    pub fn into_current_queue(&self) -> CurrentQueue {
        CurrentQueue::new(
            remove,
            AddFn::Normal(add),
            AtomicPtr::new(self as *const _ as *mut _),
        )
    }
}

impl PartialEq for QueuedThread {
    fn eq(&self, other: &Self) -> bool {
        match self {
            QueuedThread::Actual(us) => match other {
                QueuedThread::Actual(other) => us == other,
                QueuedThread::Compare(ptr) => us.matching(*ptr),
            },
            QueuedThread::Compare(us) => match other {
                QueuedThread::Actual(other) => other.matching(*us),
                QueuedThread::Compare(ptr) => *us == *ptr,
            },
        }
    }
}

impl<I: PartialOrd + Copy> SortedThreadQueue<I> {
    pub const fn new() -> Self {
        SortedThreadQueue(CriticalLock::new(SortedQueue::new()))
    }

    pub fn insert(&self, thread: ThreadOwner, value: I) {
        self.0.lock().insert(QueuedThread::Actual(thread), value)
    }

    #[allow(unused)]
    pub fn pop(&self) -> Option<ThreadOwner> {
        self.0.lock().pop().map(|queued| match queued {
            QueuedThread::Actual(thread) => thread,
            _ => panic!("\"Compare\" queued thread should never actually be in the queue!"),
        })
    }

    pub fn pop_le(&self, value: I) -> Option<ThreadOwner> {
        self.0.lock().pop_le(value).map(|queued| match queued {
            QueuedThread::Actual(thread) => thread,
            _ => panic!("\"Compare\" queued thread should never actually be in the queue!"),
        })
    }

    pub unsafe fn remove(&self, thread: *const ThreadInner) {
        self.0.lock().remove(QueuedThread::Compare(thread));
    }

    pub fn into_current_queue(&self, value: I) -> CurrentQueue<I> {
        CurrentQueue::new(
            remove_sorted::<I>,
            AddFn::Sorted(value, add_sorted),
            AtomicPtr::new(self as *const _ as *mut _),
        )
    }
}
