use core::mem::ManuallyDrop;

use crate::{thread::QueueAccess, CurrentQueue, ProcessTypes, Thread};
use alloc::boxed::Box;
use base::{
    critical::CriticalLock,
    multi_owner::{Owner, Reference},
    queue::{Queue, SortedQueue},
};

#[allow(unused)]
enum QueuedThread<T: ProcessTypes + 'static> {
    Actual(Owner<Thread<T>>),
    Compare(Reference<Thread<T>>),
}

pub struct ThreadQueue<T: ProcessTypes + 'static>(CriticalLock<Queue<QueuedThread<T>>>);

pub struct SortedThreadQueue<K: Ord + Copy + Send + Sync + 'static, T: ProcessTypes + 'static>(
    CriticalLock<SortedQueue<K, QueuedThread<T>>>,
);

unsafe fn add<T: ProcessTypes>(queue: *mut core::ffi::c_void, thread: Owner<Thread<T>>, _: usize) {
    let queue = &*(queue as *const ThreadQueue<T>);
    queue.push(thread)
}

unsafe fn remove<T: ProcessTypes>(
    queue: *mut core::ffi::c_void,
    thread: &Reference<Thread<T>>,
    _: usize,
) -> Option<Owner<Thread<T>>> {
    let queue = &*(queue as *const ThreadQueue<T>);
    queue.remove(thread)
}

unsafe fn drop(_: usize) {}

unsafe fn add_sorted<K: Ord + Copy + Send + Sync + Clone + 'static, T: ProcessTypes>(
    queue: *mut core::ffi::c_void,
    thread: Owner<Thread<T>>,
    context: usize,
) {
    let key = ManuallyDrop::new(Box::from_raw(context as *mut K));
    let queue = &*(queue as *const SortedThreadQueue<K, T>);
    queue.insert(thread, key.as_ref().clone())
}

unsafe fn remove_sorted<K: Ord + Copy + Send + Sync + 'static, T: ProcessTypes>(
    queue: *mut core::ffi::c_void,
    thread: &Reference<Thread<T>>,
    _: usize,
) -> Option<Owner<Thread<T>>> {
    let queue = &*(queue as *const SortedThreadQueue<K, T>);
    queue.remove(thread)
}

unsafe fn drop_sorted<K: PartialOrd + Copy + Send + Sync>(context: usize) {
    let mut key = ManuallyDrop::new(Box::from_raw(context as *mut K));
    ManuallyDrop::drop(&mut key);
}

impl<T: ProcessTypes> ThreadQueue<T> {
    pub fn new() -> Self {
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
        CurrentQueue::new(
            QueueAccess::new(0, add::<T>, remove::<T>, drop),
            self as *const _ as *mut _,
        )
    }

    pub fn length(&self) -> usize {
        self.0.lock().len()
    }
}

impl<K: Ord + Copy + Send + Sync + 'static, T: ProcessTypes> SortedThreadQueue<K, T> {
    pub fn new() -> Self {
        SortedThreadQueue(CriticalLock::new(SortedQueue::new()))
    }

    pub fn insert(&self, thread: Owner<Thread<T>>, key: K) {
        thread.lock(|thread| thread.set_current_queue(self.current_queue(key)));
        self.0.lock().insert(QueuedThread::Actual(thread), key);
    }

    pub fn pop(&self) -> Option<Owner<Thread<T>>> {
        self.0.lock().pop().map(|t| {
            let t = t.unwrap();
            t.lock(|t| unsafe { t.clear_queue(true) });
            t
        })
    }

    pub fn pop_le(&self, key: K) -> Option<Owner<Thread<T>>> {
        self.0.lock().pop_le(key).map(|queued| match queued {
            QueuedThread::Actual(thread) => thread,
            _ => panic!("\"Compare\" queued thread should never actually be in the queue!"),
        })
    }

    pub fn remove(&self, thread: &Reference<Thread<T>>) -> Option<Owner<Thread<T>>> {
        self.0
            .lock()
            .remove(QueuedThread::Compare(thread.clone()))
            .map(|thread| thread.into())
    }

    pub fn current_queue(&self, key: K) -> CurrentQueue<T> {
        let key = ManuallyDrop::new(Box::new(key));

        CurrentQueue::new(
            QueueAccess::new(
                key.as_ref() as *const _ as usize,
                add_sorted::<K, T>,
                remove_sorted::<K, T>,
                drop_sorted::<K>,
            ),
            self as *const _ as *mut _,
        )
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
