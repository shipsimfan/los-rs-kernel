use super::{inner::ThreadInner, ThreadOwner};
use core::{
    ffi::c_void,
    sync::atomic::{AtomicPtr, Ordering},
};

pub type RemoveFn = unsafe fn(*mut c_void, *const ThreadInner);
pub type AddFn = unsafe fn(*mut c_void, ThreadOwner);

pub struct CurrentQueue {
    remove: RemoveFn,
    add: AddFn,
    object: AtomicPtr<c_void>,
}

impl CurrentQueue {
    pub fn new(remove: RemoveFn, add: AddFn, object: AtomicPtr<c_void>) -> Self {
        CurrentQueue {
            remove,
            add,
            object,
        }
    }

    pub unsafe fn remove(&self, thread: *const ThreadInner) {
        (self.remove)(self.object.load(Ordering::Acquire), thread);
    }

    pub unsafe fn add(&self, thread: ThreadOwner) {
        (self.add)(self.object.load(Ordering::Acquire), thread);
    }
}

impl Clone for CurrentQueue {
    fn clone(&self) -> Self {
        CurrentQueue {
            remove: self.remove,
            add: self.add,
            object: AtomicPtr::new(self.object.load(Ordering::Acquire)),
        }
    }
}
