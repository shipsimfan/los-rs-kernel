#![no_std]
#![feature(const_fn_trait_bound)]

use alloc::{boxed::Box, string::String};
use base::{log_info, multi_owner::Reference};
use process::{Mutex, ProcessTypes};
use tree::Tree;

mod device;
mod port_io;
mod tree;
mod types;

extern crate alloc;

pub use device::*;
pub use port_io::*;
pub use types::*;

const MODULE_NAME: &str = "Device";

static mut DEVICE_INITIALIZED: bool = false;

process::static_generic!(process::Mutex<crate::tree::Tree<T>, T>, device_tree);

pub fn initialize<T: ProcessTypes + 'static>() {
    log_info!("Initializing . . .");

    unsafe {
        assert!(!DEVICE_INITIALIZED);
        DEVICE_INITIALIZED = true;
    }

    device_tree::initialize::<T>(Tree::new());

    log_info!("Initialized!");
}

pub fn register_device<T: ProcessTypes + 'static>(
    path: &str,
    device: Box<dyn Device>,
) -> base::error::Result<()> {
    let mut lock = device_tree::get::<T>().lock();

    lock.register_device(path, device)
}

pub fn get_device<T: ProcessTypes + 'static>(
    path: &str,
) -> base::error::Result<Reference<Box<dyn Device>, Mutex<Box<dyn Device>, T>>> {
    device_tree::get::<T>().lock().get_device(path)
}

pub fn get_children<T: ProcessTypes + 'static>(path: &str) -> base::error::Result<Box<[String]>> {
    device_tree::get::<T>().lock().get_children(path)
}

pub fn remove_device<T: ProcessTypes + 'static>(path: &str) {
    device_tree::get::<T>().lock().remove_device(path);
}
