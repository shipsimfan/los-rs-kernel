use super::{inner::ThreadInner, queue::CurrentQueue, ThreadReference};
use crate::{
    critical::CriticalLock,
    ipc::Signals,
    map::Mappable,
    process::{process::ProcessOwner, ProcessReference},
};
use alloc::sync::Arc;

pub struct ThreadOwner(Arc<CriticalLock<ThreadInner>>);

impl ThreadOwner {
    pub fn new(
        process: ProcessOwner,
        entry: usize,
        context: usize,
        signals: Signals,
    ) -> ThreadOwner {
        ThreadOwner(Arc::new(CriticalLock::new(ThreadInner::new(
            process, entry, context, signals,
        ))))
    }

    pub fn id(&self) -> isize {
        self.0.lock().id()
    }

    pub unsafe fn set_queue(&self, queue: CurrentQueue) {
        self.0.lock().set_queue(queue)
    }

    pub fn set_queue_data(&self, new_data: isize) {
        self.0.lock().set_queue_data(new_data)
    }

    pub fn load_float(&self) {
        self.0.lock().load_float()
    }

    pub fn set_interrupt_stack(&self) {
        self.0.lock().set_interrupt_stack()
    }

    pub fn get_stack_pointer_location(&self) -> *const usize {
        self.0.lock().get_stack_pointer_location()
    }

    pub fn get_kernel_stack_base(&self) -> usize {
        self.0.lock().get_kernel_stack_base()
    }

    pub fn process(&self) -> ProcessReference {
        self.0.lock().process()
    }

    pub fn reference(&self) -> ThreadReference {
        ThreadReference::new(Arc::downgrade(&self.0))
    }

    pub fn matching(&self, thread: *const ThreadInner) -> bool {
        (&(*self.0.lock())) as *const _ == thread
    }
}

impl PartialEq for ThreadOwner {
    fn eq(&self, other: &Self) -> bool {
        self.0.as_ref() == other.0.as_ref()
    }
}
