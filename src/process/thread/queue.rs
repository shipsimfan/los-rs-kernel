use alloc::boxed::Box;

use super::{inner::ThreadInner, ThreadOwner};
use core::{
    ffi::c_void,
    sync::atomic::{AtomicPtr, Ordering},
};

pub type RemoveFn = unsafe fn(*mut c_void, *const ThreadInner) -> Option<ThreadOwner>;

pub trait AddFn: Send + Sync {
    unsafe fn add(&self, queue: *mut c_void, thread: ThreadOwner);
}

pub struct CurrentQueue {
    remove: RemoveFn,
    add: Box<dyn AddFn>,
    object: AtomicPtr<c_void>,
}

impl CurrentQueue {
    pub fn new(remove: RemoveFn, add: Box<dyn AddFn>, object: AtomicPtr<c_void>) -> Self {
        CurrentQueue {
            remove,
            add,
            object,
        }
    }

    pub unsafe fn remove(&self, thread: *const ThreadInner) -> Option<ThreadOwner> {
        (self.remove)(self.object.load(Ordering::Acquire), thread)
    }

    pub unsafe fn add(&self, thread: ThreadOwner) {
        self.add.add(self.object.load(Ordering::Acquire), thread)
    }
}
