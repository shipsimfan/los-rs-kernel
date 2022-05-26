#![no_std]

use alloc::boxed::Box;
use base::{log_error, log_info};
use process::ProcessTypes;
use sessions::Session;

mod controller;
mod keyboard;

extern crate alloc;

const MODULE_NAME: &str = "PS/2";

static mut PS2_INITIALIZED: bool = false;

pub fn initialize<T: ProcessTypes<Owner = Box<dyn Session<T>>> + 'static>() {
    log_info!("Initializing . . . ");

    unsafe {
        assert!(!PS2_INITIALIZED);
        PS2_INITIALIZED = true;
    }

    // Determine if a PS/2 controller exists
    let fadt = match acpi::get_table::<acpi::FADT, T>() {
        Some(table) => table,
        None => return log_error!("Unable to get FADT table"),
    };

    if fadt.boot_architecture_flags & 2 == 0 {
        return log_error!("No PS/2 controller found");
    }

    // Create the controller
    let controller = controller::Controller::<T>::new();

    // Register the controller
    match device::register_device::<T>("/ps2", controller.clone()) {
        Ok(()) => {}
        Err(status) => return log_error!("Failed to register PS/2 controller: {}", status),
    }

    // Enumerate devices on the controller
    match controller.lock(|controller| controller.ioctrl(0, 0)) {
        Ok(_) => {}
        Err(status) => return log_error!("Failed to enumerate PS/2 devices: {}", status),
    }

    log_info!("Initialized!");
}
