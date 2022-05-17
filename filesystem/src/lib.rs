#![no_std]
#![feature(associated_type_bounds)]

use base::log_info;
use process::ProcessTypes;

mod directory;
mod drive;
mod driver;
mod file;
mod functions;
mod metadata;
mod mount;
mod volume;

extern crate alloc;

pub use directory::*;
pub use drive::{get_drives, register_drive, unregister_drive, Drive};
pub use driver::{register_filesystem_driver, DetectFilesystemFunction};
pub use file::*;
pub use functions::*;
pub use metadata::*;
pub use mount::{get_volume, mount_volume, unmount_volume};
pub use volume::*;

const MODULE_NAME: &str = "Filesystem";

static mut FILESYSTEM_INITIALIZED: bool = false;

pub fn initialize<T: ProcessTypes + 'static>() {
    log_info!("Initializing . . .");

    unsafe {
        assert!(!FILESYSTEM_INITIALIZED);
        FILESYSTEM_INITIALIZED = true;
    }

    drive::initialize::<T>();
    driver::initialize::<T>();
    mount::initialize::<T>();

    log_info!("Initialized!");
}
