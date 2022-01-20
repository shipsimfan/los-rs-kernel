use super::{inner::ProcessInner, ProcessReference};
use crate::{
    critical::{CriticalLock, CriticalLockGuard},
    filesystem::DirectoryDescriptor,
    process::ThreadOwner,
};
use alloc::sync::Arc;

#[derive(Clone)]
pub struct ProcessOwner(Arc<CriticalLock<ProcessInner>>);

impl ProcessOwner {
    pub fn new(
        session_id: Option<isize>,
        current_working_directory: Option<DirectoryDescriptor>,
    ) -> Self {
        ProcessOwner(Arc::new(CriticalLock::new(ProcessInner::new(
            session_id,
            current_working_directory,
        ))))
    }

    pub fn new_raw(process: Arc<CriticalLock<ProcessInner>>) -> Self {
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

    pub fn lock(&self) -> CriticalLockGuard<ProcessInner> {
        self.0.lock()
    }
}
