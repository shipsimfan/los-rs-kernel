use crate::Volume;
use alloc::{boxed::Box, vec::Vec};
use base::{log_info, multi_owner::Owner};
use device::Device;
use process::{Mutex, MutexGuard, ProcessTypes};

pub type DetectFilesystemFunction<T> = fn(
    drive: &Owner<Box<dyn Device>, Mutex<Box<dyn Device>, T>>,
    start: usize,
    size: usize,
) -> base::error::Result<Option<Volume<T>>>;

process::static_generic!(
    process::Mutex<alloc::vec::Vec<crate::DetectFilesystemFunction<T>>, T>,
    filesystem_drivers
);

pub fn initialize<T: ProcessTypes + 'static>() {
    log_info!("Initializing filesystem drivers . . .");

    filesystem_drivers::initialize::<T>(Mutex::new(Vec::new()));

    log_info!("Initialized filesystem drivers!")
}

pub fn register_filesystem_driver<T: ProcessTypes + 'static>(driver: DetectFilesystemFunction<T>) {
    log_info!("Registering new filesystem driver");
    filesystem_drivers::get().lock().push(driver);
}

pub fn get_drivers<T: ProcessTypes + 'static>(
) -> MutexGuard<'static, Vec<DetectFilesystemFunction<T>>, T> {
    filesystem_drivers::get().lock()
}
