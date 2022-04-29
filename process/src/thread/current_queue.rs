use crate::{process::Signals, ProcessOwner, Thread};
use alloc::boxed::Box;
use base::multi_owner::{Owner, Reference};
use core::ffi::c_void;

pub trait QueueAccess<O: ProcessOwner<D, S> + 'static, D: 'static, S: Signals + 'static> {
    unsafe fn add(&self, queue: *mut c_void, thread: Owner<Thread<O, D, S>>);
    unsafe fn remove(
        &self,
        queue: *mut c_void,
        thread: &Reference<Thread<O, D, S>>,
    ) -> Option<Owner<Thread<O, D, S>>>;
}

pub struct CurrentQueue<O: ProcessOwner<D, S> + 'static, D: 'static, S: Signals + 'static> {
    access: Box<dyn QueueAccess<O, D, S>>,
    queue: *mut c_void,
}

impl<O: ProcessOwner<D, S>, D, S: Signals> CurrentQueue<O, D, S> {
    pub fn new(access: Box<dyn QueueAccess<O, D, S>>, queue: *mut c_void) -> Self {
        CurrentQueue { access, queue }
    }

    pub unsafe fn add(&self, thread: Owner<Thread<O, D, S>>) {
        self.access.add(self.queue, thread)
    }

    pub unsafe fn remove(
        &self,
        thread: &Reference<Thread<O, D, S>>,
    ) -> Option<Owner<Thread<O, D, S>>> {
        self.access.remove(self.queue, thread)
    }
}
