use super::DirectoryOwner;
use crate::locks::{Mutex, MutexGuard};
use alloc::sync::Arc;

#[derive(Clone)]
pub struct DirectoryReference(Arc<Mutex<DirectoryOwner>>);

impl DirectoryReference {
    pub fn new(directory: DirectoryOwner) -> Self {
        DirectoryReference(Arc::new(Mutex::new(directory)))
    }

    pub fn as_arc(&self) -> &Arc<Mutex<DirectoryOwner>> {
        &self.0
    }

    pub fn lock(&self) -> MutexGuard<DirectoryOwner> {
        self.0.lock()
    }

    pub fn matching_data(&self, other: *const DirectoryOwner) -> bool {
        self.0.matching_data(other)
    }
}
