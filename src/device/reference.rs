use super::Device;
use crate::locks::{Mutex, MutexGuard};
use alloc::{boxed::Box, sync::Arc};

#[derive(Clone)]
pub struct DeviceReference(Arc<Mutex<Box<dyn Device>>>);

impl DeviceReference {
    pub fn new(inner: Box<dyn Device>) -> Self {
        DeviceReference(Arc::new(Mutex::new(inner)))
    }

    pub unsafe fn as_ptr(&self) -> *mut Box<dyn Device> {
        self.0.as_ptr()
    }

    pub fn lock(&self) -> MutexGuard<Box<dyn Device>> {
        self.0.lock()
    }
}
