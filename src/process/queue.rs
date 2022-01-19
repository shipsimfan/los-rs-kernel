use super::{CurrentQueue, ThreadInner, ThreadOwner};
use crate::{locks::Spinlock, queue::Queue};
use core::{ffi::c_void, sync::atomic::AtomicPtr};

enum QueuedThread {
    Actual(ThreadOwner),
    Compare(*const ThreadInner),
}

pub struct ThreadQueue(Spinlock<Queue<QueuedThread>>);

unsafe fn remove(queue: *mut c_void, thread: *const ThreadInner) {
    let queue = &mut *(queue as *mut ThreadQueue);
    queue.remove(thread);
}

unsafe fn add(queue: *mut c_void, thread: ThreadOwner) {
    let queue = &mut *(queue as *mut ThreadQueue);
    queue.push(thread)
}

impl ThreadQueue {
    pub const fn new() -> Self {
        ThreadQueue(Spinlock::new(Queue::new()))
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
        CurrentQueue::new(remove, add, AtomicPtr::new(self as *const _ as *mut _))
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
