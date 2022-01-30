use super::FileOwner;
use crate::locks::{Mutex, MutexGuard};
use alloc::sync::Arc;

#[derive(Clone)]
pub struct FileReference(Arc<Mutex<FileOwner>>);

impl FileReference {
    pub fn new(file: FileOwner) -> Self {
        FileReference(Arc::new(Mutex::new(file)))
    }

    pub fn as_ptr(&self) -> *const Mutex<FileOwner> {
        Arc::as_ptr(&self.0)
    }

    pub fn lock(&self) -> MutexGuard<FileOwner> {
        self.0.lock()
    }

    pub fn matching_data(&self, other: *const FileOwner) -> bool {
        self.0.matching_data(other)
    }
}
