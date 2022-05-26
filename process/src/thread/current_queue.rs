use crate::{ProcessTypes, Thread};
use base::multi_owner::{Owner, Reference};
use core::ffi::c_void;

pub struct QueueAccess<T: ProcessTypes + 'static> {
    context: usize,
    add: unsafe fn(queue: *mut c_void, thread: Owner<Thread<T>>, context: usize),
    remove: unsafe fn(
        queue: *mut c_void,
        thread: &Reference<Thread<T>>,
        context: usize,
    ) -> Option<Owner<Thread<T>>>,
    drop: unsafe fn(context: usize),
}

pub struct CurrentQueue<T: ProcessTypes + 'static> {
    access: QueueAccess<T>,
    queue: *mut c_void,
}

impl<T: ProcessTypes> CurrentQueue<T> {
    pub fn new(access: QueueAccess<T>, queue: *mut c_void) -> Self {
        CurrentQueue { access, queue }
    }

    pub unsafe fn add(&self, thread: Owner<Thread<T>>) {
        (self.access.add)(self.queue, thread, self.access.context)
    }

    pub unsafe fn remove(&self, thread: &Reference<Thread<T>>) -> Option<Owner<Thread<T>>> {
        (self.access.remove)(self.queue, thread, self.access.context)
    }
}

impl<T: ProcessTypes> QueueAccess<T> {
    pub fn new(
        context: usize,
        add: unsafe fn(queue: *mut c_void, thread: Owner<Thread<T>>, context: usize),
        remove: unsafe fn(
            queue: *mut c_void,
            thread: &Reference<Thread<T>>,
            context: usize,
        ) -> Option<Owner<Thread<T>>>,
        drop: unsafe fn(context: usize),
    ) -> Self {
        QueueAccess {
            context,
            add,
            remove,
            drop,
        }
    }
}

impl<T: ProcessTypes> Drop for QueueAccess<T> {
    fn drop(&mut self) {
        unsafe { (self.drop)(self.context) }
    }
}

impl<T: ProcessTypes> Clone for QueueAccess<T> {
    fn clone(&self) -> Self {
        QueueAccess {
            context: self.context,
            add: self.add,
            remove: self.remove,
            drop: self.drop,
        }
    }
}

impl<T: ProcessTypes> Clone for CurrentQueue<T> {
    fn clone(&self) -> Self {
        CurrentQueue {
            access: self.access.clone(),
            queue: self.queue,
        }
    }
}
