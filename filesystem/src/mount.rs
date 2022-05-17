use crate::Volume;
use alloc::boxed::Box;
use base::{error::FILESYSTEM_MODULE_NUMBER, hash_map::HashMap, log_info, multi_owner::Owner};
use process::{Mutex, ProcessTypes};

#[derive(Debug)]
pub enum MountError {
    AlreadyMounted,
    BootDriveUnmount,
}

process::static_generic!(
    process::Mutex<
        base::hash_map::HashMap<
            usize,
            base::multi_owner::Owner<crate::Volume<T>, process::Mutex<crate::Volume<T>, T>>,
        >,
        T,
    >,
    mounts
);

pub fn initialize<T: ProcessTypes + 'static>() {
    log_info!("Initializing mounts . . .");

    mounts::initialize::<T>(Mutex::new(HashMap::new()));

    log_info!("Initialized mounts!");
}

pub fn mount_volume<T: ProcessTypes + 'static>(
    volume: Owner<Volume<T>, Mutex<Volume<T>, T>>,
    mount_point: usize,
) -> base::error::Result<()> {
    if !mounts::get::<T>()
        .lock()
        .try_insert(mount_point, volume.clone())
    {
        return Err(Box::new(MountError::AlreadyMounted));
    }

    volume.lock(|volume| unsafe { volume.set_mount_point(Some(mount_point)) });

    log_info!("Mounted volume at {}", mount_point);

    Ok(())
}

pub fn unmount_volume<T: ProcessTypes + 'static>(mount_point: usize) -> base::error::Result<()> {
    if mount_point == 0 {
        return Err(Box::new(MountError::BootDriveUnmount));
    }

    match mounts::get::<T>().lock().remove(&mount_point) {
        Some(volume) => volume.lock(|volume| unsafe { volume.set_mount_point(None) }),
        None => {}
    }

    Ok(())
}

pub fn get_volume<T: ProcessTypes + 'static>(
    mount_point: usize,
) -> Option<Owner<Volume<T>, Mutex<Volume<T>, T>>> {
    mounts::get::<T>()
        .lock()
        .get(&mount_point)
        .map(|volume| volume.clone())
}

impl base::error::Error for MountError {
    fn module_number(&self) -> i32 {
        FILESYSTEM_MODULE_NUMBER
    }

    fn error_number(&self) -> base::error::Status {
        match self {
            MountError::AlreadyMounted => base::error::Status::Exists,
            MountError::BootDriveUnmount => base::error::Status::InvalidArgument,
        }
    }
}

impl core::fmt::Display for MountError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            MountError::AlreadyMounted => write!(f, "Volume already mounted at mount point"),
            MountError::BootDriveUnmount => write!(f, "Unable to unmount the boot drive"),
        }
    }
}
