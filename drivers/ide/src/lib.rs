#![no_std]

use alloc::boxed::Box;
use base::{log_error, log_info, multi_owner::Owner};
use constants::{IDE_PATH, PCI_IDE_PATH};
use device::Device;
use process::ProcessTypes;

extern crate alloc;

mod constants;
mod controller;
mod pio;

const MODULE_NAME: &str = "IDE";

static mut IDE_INITIALIZED: bool = false;

fn get_base_address_registers(
    pci_device: &mut Box<dyn Device>,
) -> base::error::Result<(usize, usize, usize, usize, usize)> {
    let bar0 = pci_device.read_register(pci::Register::BAR0 as usize)?;
    let bar1 = pci_device.read_register(pci::Register::BAR1 as usize)?;
    let bar2 = pci_device.read_register(pci::Register::BAR2 as usize)?;
    let bar3 = pci_device.read_register(pci::Register::BAR3 as usize)?;
    let bar4 = pci_device.read_register(pci::Register::BAR4 as usize)?;
    Ok((bar0, bar1, bar2, bar3, bar4))
}

pub fn initialize<T: ProcessTypes + 'static>() {
    log_info!("Initializing . . . ");

    unsafe {
        assert!(!IDE_INITIALIZED);
        IDE_INITIALIZED = true;
    }

    // Get the PCI IDE device
    let pci_device_lock = match device::get_device::<T>(PCI_IDE_PATH) {
        Ok(device) => device,
        Err(status) => return log_error!("\nError while getting PCI IDE device: {}", status),
    };

    // Get the BARs
    let (bar0, bar1, bar2, bar3, bar4) = match pci_device_lock
        .lock(|pci_device| get_base_address_registers(pci_device))
        .unwrap()
    {
        Ok(bars) => bars,
        Err(status) => {
            return log_error!("\nError while getting base address registers: {}", status)
        }
    };

    // Remove the pci device
    drop(pci_device_lock);
    device::remove_device::<T>(PCI_IDE_PATH);

    // Create and register the IDE Controller
    match device::register_device::<T>(
        IDE_PATH,
        Owner::new(Box::new(controller::IDEController::<T>::new(
            bar0, bar1, bar2, bar3, bar4,
        )) as Box<dyn Device>),
    ) {
        Ok(()) => {}
        Err(status) => return log_error!("\nError while registering IDE controller: {}", status),
    }

    // Enumerate drives
    let ide_controller_lock = match device::get_device::<T>(IDE_PATH) {
        Ok(device) => device,
        Err(status) => return log_error!("\nError while getting IDE controller: {}", status),
    };
    match ide_controller_lock
        .lock(|controller| controller.ioctrl(controller::IOCTRL_ENUMERATE, 0))
        .unwrap()
    {
        Ok(_) => {}
        Err(status) => log_error!("Error: {}!", status),
    }

    // Register drives
    match device::get_device::<T>("/ide/primary_master") {
        Ok(device) => {
            match device.lock(|device| device.ioctrl(0, 0)) {
                Some(size) => {
                    match filesystem::register_drive::<T>("/ide/primary_master", size.unwrap()) {
                        Ok(()) => {}
                        Err(status) => {
                            log_error!("Error while registering primary master: {}", status)
                        }
                    }
                }
                None => {}
            };
        }
        Err(_) => {}
    }

    match device::get_device::<T>("/ide/primary_slave") {
        Ok(device) => {
            match device.lock(|device| device.ioctrl(0, 0)) {
                Some(size) => {
                    match filesystem::register_drive::<T>("/ide/primary_slave", size.unwrap()) {
                        Ok(()) => {}
                        Err(status) => {
                            log_error!("Error while registering primary slave: {}", status)
                        }
                    }
                }
                None => {}
            };
        }
        Err(_) => {}
    }

    match device::get_device::<T>("/ide/secondary_master") {
        Ok(device) => {
            match device.lock(|device| device.ioctrl(0, 0)) {
                Some(size) => {
                    match filesystem::register_drive::<T>("/ide/secondary_master", size.unwrap()) {
                        Ok(()) => {}
                        Err(status) => {
                            log_error!("Error while registering secondary master: {}", status)
                        }
                    }
                }
                None => {}
            };
        }
        Err(_) => {}
    }

    match device::get_device::<T>("/ide/secondary_slave") {
        Ok(device) => {
            match device.lock(|device| device.ioctrl(0, 0)) {
                Some(size) => {
                    match filesystem::register_drive::<T>("/ide/secondary_slave", size.unwrap()) {
                        Ok(()) => {}
                        Err(status) => {
                            log_error!("Error while registering secondary slave: {}", status)
                        }
                    }
                }
                None => {}
            };
        }
        Err(_) => {}
    }

    log_info!("Initialized!");
}
