#![no_std]
#![feature(const_fn_trait_bound)]

use alloc::{boxed::Box, string::String};
use base::multi_owner::Reference;
use core::{ffi::c_void, mem::ManuallyDrop, ptr::null};
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

type DeviceTreeType<T> = &'static Mutex<Tree<T>, T>;

static mut DEVICE_INITIALIZED: bool = false;
static mut DEVICE_TREE_PTR: *const c_void = null();

fn device_tree<T: ProcessTypes + 'static>() -> DeviceTreeType<T> {
    unsafe { &*(DEVICE_TREE_PTR as *const _) }
}

pub fn initialize<T: ProcessTypes + 'static>() {
    unsafe {
        assert!(!DEVICE_INITIALIZED);
        DEVICE_INITIALIZED = true;

        assert_eq!(DEVICE_TREE_PTR, null());

        let tree = ManuallyDrop::new(Box::new(Tree::<T>::new()));
        DEVICE_TREE_PTR = tree.as_ref() as *const _ as *const _;
    }
}

pub fn register_device<T: ProcessTypes + 'static>(
    path: &str,
    device: Box<dyn Device>,
) -> base::error::Result<()> {
    let mut lock = device_tree::<T>().lock();

    lock.register_device(path, device)
}

pub fn get_device<T: ProcessTypes + 'static>(
    path: &str,
) -> base::error::Result<Reference<Box<dyn Device>, Mutex<Box<dyn Device>, T>>> {
    device_tree::<T>().lock().get_device(path)
}

pub fn get_children<T: ProcessTypes + 'static>(path: &str) -> base::error::Result<Box<[String]>> {
    device_tree::<T>().lock().get_children(path)
}

pub fn remove_device<T: ProcessTypes + 'static>(path: &str) {
    device_tree::<T>().lock().remove_device(path);
}
