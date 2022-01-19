use super::{inner::ProcessInner, ProcessReference};
use crate::{
    filesystem::DirectoryDescriptor,
    locks::{Spinlock, SpinlockGuard},
    process::ThreadOwner,
};
use alloc::sync::Arc;

#[derive(Clone)]
pub struct ProcessOwner(Arc<Spinlock<ProcessInner>>);

impl ProcessOwner {
    pub fn new(
        session_id: Option<isize>,
        current_working_directory: Option<DirectoryDescriptor>,
    ) -> Self {
        ProcessOwner(Arc::new(Spinlock::new(ProcessInner::new(
            session_id,
            current_working_directory,
        ))))
    }

    pub fn new_raw(process: Arc<Spinlock<ProcessInner>>) -> Self {
        ProcessOwner(process)
    }

    pub fn reference(&self) -> ProcessReference {
        ProcessReference::new(Arc::downgrade(&self.0))
    }

    pub fn remove_thread(&self, tid: isize) -> bool {
        self.0.lock().remove_thread(tid)
    }

    pub fn create_thread(self, entry: usize, context: usize) -> ThreadOwner {
        self.0.lock().create_thread(entry, context, self.clone())
    }

    pub fn lock(&self) -> SpinlockGuard<ProcessInner> {
        self.0.lock()
    }
}
