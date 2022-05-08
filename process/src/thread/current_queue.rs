use crate::{ProcessTypes, Thread};
use alloc::boxed::Box;
use base::multi_owner::{Owner, Reference};
use core::ffi::c_void;

pub trait QueueAccess<T: ProcessTypes> {
    unsafe fn add(&self, queue: *mut c_void, thread: Owner<Thread<T>>);
    unsafe fn remove(
        &self,
        queue: *mut c_void,
        thread: &Reference<Thread<T>>,
    ) -> Option<Owner<Thread<T>>>;
}

pub struct CurrentQueue<T: ProcessTypes> {
    access: Box<dyn QueueAccess<T>>,
    queue: *mut c_void,
}

impl<T: ProcessTypes> CurrentQueue<T> {
    pub fn new(access: Box<dyn QueueAccess<T>>, queue: *mut c_void) -> Self {
        CurrentQueue { access, queue }
    }

    pub unsafe fn add(&self, thread: Owner<Thread<T>>) {
        self.access.add(self.queue, thread)
    }

    pub unsafe fn remove(&self, thread: &Reference<Thread<T>>) -> Option<Owner<Thread<T>>> {
        self.access.remove(self.queue, thread)
    }
}
