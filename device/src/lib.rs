#![no_std]
#![feature(const_fn_trait_bound)]

use alloc::{boxed::Box, string::String};
use base::multi_owner::Reference;
use core::{ffi::c_void, mem::ManuallyDrop, ptr::null};
use process::{Mutex, ProcessOwner, Signals};
use tree::Tree;

mod device;
mod port_io;
mod tree;
mod types;

extern crate alloc;

pub use device::*;
pub use port_io::*;
pub use types::*;

type DeviceTreeType<O, D, S> = &'static Mutex<Tree<O, D, S>, O, D, S>;

static mut DEVICE_INITIALIZED: bool = false;
static mut DEVICE_TREE_PTR: *const c_void = null();

fn device_tree<O: ProcessOwner<D, S> + 'static, D: 'static, S: Signals + 'static>(
) -> DeviceTreeType<O, D, S> {
    let r = unsafe { &*(DEVICE_TREE_PTR as *const _) };

    r
}

pub fn initialize<O: ProcessOwner<D, S> + 'static, D: 'static, S: Signals + 'static>() {
    unsafe {
        assert!(!DEVICE_INITIALIZED);
        DEVICE_INITIALIZED = true;

        assert_eq!(DEVICE_TREE_PTR, null());

        let tree = ManuallyDrop::new(Box::new(Tree::<O, D, S>::new()));
        DEVICE_TREE_PTR = tree.as_ref() as *const _ as *const _;
    }
}

pub fn register_device<O: ProcessOwner<D, S> + 'static, D: 'static, S: Signals + 'static>(
    path: &str,
    device: Box<dyn Device>,
) -> base::error::Result<()> {
    let mut lock = device_tree::<O, D, S>().lock();

    lock.register_device(path, device)
}

pub fn get_device<O: ProcessOwner<D, S> + 'static, D: 'static, S: Signals + 'static>(
    path: &str,
) -> base::error::Result<Reference<Box<dyn Device>, Mutex<Box<dyn Device>, O, D, S>>> {
    device_tree::<O, D, S>().lock().get_device(path)
}

pub fn get_children<O: ProcessOwner<D, S> + 'static, D: 'static, S: Signals + 'static>(
    path: &str,
) -> base::error::Result<Box<[String]>> {
    device_tree::<O, D, S>().lock().get_children(path)
}

pub fn remove_device<O: ProcessOwner<D, S> + 'static, D: 'static, S: Signals + 'static>(
    path: &str,
) {
    device_tree::<O, D, S>().lock().remove_device(path);
}
