use crate::{driver, Volume};
use alloc::{borrow::ToOwned, boxed::Box, string::String, vec::Vec};
use base::{error::FILESYSTEM_MODULE_NUMBER, log_info, map::Map, multi_owner::Owner};
use process::{Mutex, ProcessTypes};

pub struct Drive<T: ProcessTypes + 'static> {
    device_path: String,
    volumes: Map<Owner<Volume<T>, Mutex<Volume<T>, T>>>,
}

#[derive(Debug)]
struct DriveAlreadyRegisteredError;

process::static_generic!(
    process::Mutex<
        alloc::vec::Vec<
            base::multi_owner::Owner<crate::Drive<T>, process::Mutex<crate::Drive<T>, T>>,
        >,
        T,
    >,
    drives
);

pub fn initialize<T: ProcessTypes + 'static>() {
    log_info!("Initializing drives . . .");

    drives::initialize::<T>(Mutex::new(Vec::new()));

    log_info!("Initialized drives!");
}

pub fn register_drive<T: ProcessTypes + 'static>(
    path: &str,
    size: usize,
) -> base::error::Result<()> {
    log_info!("Registering \"{}\" as a drive", path);

    let mut drives = drives::get::<T>().lock();

    // Verify the drive isn't already registered
    for drive in &*drives {
        if drive.lock(|drive| drive.device_path == path) {
            return Err(Box::new(DriveAlreadyRegisteredError));
        }
    }

    // Get the device
    let drive = device::get_device::<T>(path)?.upgrade();

    // Search for volumes
    let mut volumes = Map::new();
    let drivers = driver::get_drivers::<T>();
    for driver in &*drivers {
        match driver(&drive, 0, size)? {
            Some(volume) => {
                volumes.insert(Owner::new(volume));
                break;
            }
            None => {}
        };
    }

    // Insert the drive
    drives.push(Owner::new(Drive {
        device_path: path.to_owned(),
        volumes,
    }));

    Ok(())
}

pub fn unregister_drive<T: ProcessTypes + 'static>(path: &str) -> base::error::Result<()> {
    log_info!("Unregistering \"{}\" as a drive", path);

    drives::get::<T>().lock().retain(|drive| {
        drive.lock(|drive| {
            for volume in drive.volumes.iter() {
                match volume.lock(|volume| volume.mount_point()) {
                    Some(mount_point) => match crate::unmount_volume::<T>(mount_point) {
                        Ok(()) => {}
                        Err(_) => panic!("Attempting to unregister the boot drive!"),
                    },
                    None => {}
                }
            }

            drive.device_path != path
        })
    });

    Ok(())
}

pub fn get_drives<T: ProcessTypes + 'static>() -> Box<[String]> {
    drives::get::<T>()
        .lock()
        .iter()
        .map(|drive| drive.lock(|drive| drive.device_path.clone()))
        .collect()
}

impl<T: ProcessTypes + 'static> Drive<T> {
    pub fn path(&self) -> &str {
        &self.device_path
    }

    pub fn volumes(&self) -> &Map<Owner<Volume<T>, Mutex<Volume<T>, T>>> {
        &self.volumes
    }
}

impl base::error::Error for DriveAlreadyRegisteredError {
    fn module_number(&self) -> i32 {
        FILESYSTEM_MODULE_NUMBER
    }

    fn error_number(&self) -> base::error::Status {
        base::error::Status::Exists
    }
}

impl core::fmt::Display for DriveAlreadyRegisteredError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Drive already registered")
    }
}
