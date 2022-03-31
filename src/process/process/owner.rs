use super::{inner::ProcessInner, ProcessReference};
use crate::{
    critical::{CriticalLock, CriticalLockGuard},
    filesystem::DirectoryDescriptor,
    ipc::Signals,
    process::ThreadOwner,
};
use alloc::{boxed::Box, string::String, sync::Arc};

#[derive(Clone)]
pub struct ProcessOwner(Arc<CriticalLock<Box<ProcessInner>>>);

impl ProcessOwner {
    pub fn new(
        session_id: Option<isize>,
        current_working_directory: Option<DirectoryDescriptor>,
        name: String,
        signals: Signals,
    ) -> Self {
        ProcessOwner(ProcessInner::new(
            session_id,
            current_working_directory,
            name,
            signals,
        ))
    }

    pub fn new_raw(process: Arc<CriticalLock<Box<ProcessInner>>>) -> Self {
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

    pub fn lock(&self) -> CriticalLockGuard<Box<ProcessInner>> {
        self.0.lock()
    }
}