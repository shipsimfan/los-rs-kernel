use crate::{
    device::{self, acpi, DeviceBox},
    locks::Mutex,
    log, logln,
};
use alloc::{boxed::Box, sync::Arc};

mod controller;
mod keyboard;

pub fn initialize() {
    log!("Initializing PS/2 . . . ");

    // Determine if a PS/2 controller exists
    let fadt: &acpi::FADT = match acpi::get_table() {
        Ok(table) => table,
        Err(err) => return logln!("\nUnable to get FADT: {}", err),
    };

    if fadt.boot_architecture_flags & 2 == 0 {
        return logln!("\nNo PS/2 controller found");
    }

    // Create the controller
    let controller: DeviceBox = Arc::new(Mutex::new(Box::new(controller::Controller::new())));

    // Register the controller
    match device::register_device("/ps2", controller.clone()) {
        Ok(()) => {}
        Err(status) => return logln!("Failed to register PS/2 controller: {}", status),
    }

    // Enumerate devices on the controller
    match controller.lock().ioctrl(0, 0) {
        Ok(_) => {}
        Err(status) => return logln!("Failed to enumerate PS/2 devices: {}", status),
    }

    logln!("\x1B2A2]OK\x1B]!");
}
