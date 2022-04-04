use super::{inner::ThreadInner, ThreadOwner};
use core::{
    ffi::c_void,
    sync::atomic::{AtomicPtr, Ordering},
};

pub type RemoveFn = unsafe fn(*mut c_void, *const ThreadInner) -> Option<ThreadOwner>;

#[derive(Clone, Copy)]
pub enum AddFn<I: PartialOrd + Copy> {
    Normal(unsafe fn(*mut c_void, ThreadOwner)),
    Sorted(I, unsafe fn(*mut c_void, ThreadOwner, value: I)),
}

pub struct CurrentQueue<I = usize>
where
    I: PartialOrd + Copy,
{
    remove: RemoveFn,
    add: AddFn<I>,
    object: AtomicPtr<c_void>,
}

impl<I: PartialOrd + Copy> CurrentQueue<I> {
    pub fn new(remove: RemoveFn, add: AddFn<I>, object: AtomicPtr<c_void>) -> Self {
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
        match self.add {
            AddFn::Normal(add) => (add)(self.object.load(Ordering::Acquire), thread),
            AddFn::Sorted(value, add) => (add)(self.object.load(Ordering::Acquire), thread, value),
        }
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
