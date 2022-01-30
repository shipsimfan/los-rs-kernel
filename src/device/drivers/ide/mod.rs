use super::pci;
use crate::{
    device::{
        self,
        drivers::ide::constants::{IDE_PATH, PCI_IDE_PATH},
        Device, DeviceReference,
    },
    error, filesystem, log, logln,
};
use alloc::boxed::Box;

mod ata;
mod atapi;
mod constants;
mod controller;

fn get_base_address_registers(
    pci_device: &mut Box<dyn Device>,
) -> error::Result<(usize, usize, usize, usize, usize)> {
    let bar0 = pci_device.read_register(pci::Register::BAR0 as usize)?;
    let bar1 = pci_device.read_register(pci::Register::BAR1 as usize)?;
    let bar2 = pci_device.read_register(pci::Register::BAR2 as usize)?;
    let bar3 = pci_device.read_register(pci::Register::BAR3 as usize)?;
    let bar4 = pci_device.read_register(pci::Register::BAR4 as usize)?;
    Ok((bar0, bar1, bar2, bar3, bar4))
}

pub fn initialize() {
    log!("Initializing IDE . . . ");

    // Get the PCI IDE device
    let pci_device_lock = match device::get_device(PCI_IDE_PATH) {
        Ok(device) => device,
        Err(status) => return logln!("\nError while getting PCI IDE device: {}", status),
    };
    let mut pci_device = pci_device_lock.lock();

    // Get the BARs
    let (bar0, bar1, bar2, bar3, bar4) = match get_base_address_registers(&mut pci_device) {
        Ok(bars) => bars,
        Err(status) => return logln!("\nError while getting base address registers: {}", status),
    };

    // Remove the pci device
    drop(pci_device);
    drop(pci_device_lock);
    device::remove_device(PCI_IDE_PATH);

    // Create and register the IDE Controller
    match device::register_device(
        IDE_PATH,
        DeviceReference::new(Box::new(controller::IDEController::new(
            bar0, bar1, bar2, bar3, bar4,
        ))),
    ) {
        Ok(()) => {}
        Err(status) => return logln!("\nError while registering IDE controller: {}", status),
    }

    // Enumerate drives
    let ide_controller_lock = match device::get_device(IDE_PATH) {
        Ok(device) => device,
        Err(status) => return logln!("\nError while getting IDE controller: {}", status),
    };
    let mut controller = ide_controller_lock.lock();
    match controller.ioctrl(controller::IOCTRL_ENUMERATE, 0) {
        Ok(_) => logln!("OK!"),
        Err(status) => logln!("Error: {}!", status),
    }

    // Register drives
    drop(controller);

    match filesystem::register_drive("/ide/primary_master") {
        Err(status) => match status {
            error::Status::NoDevice => {}
            _ => logln!("Error while registering primary master: {}", status),
        },
        Ok(()) => {}
    }
    match filesystem::register_drive("/ide/primary_slave") {
        Err(status) => match status {
            error::Status::NoDevice => {}
            _ => logln!("Error while registering primary slave: {}", status),
        },
        Ok(()) => {}
    }
    match filesystem::register_drive("/ide/secondary_master") {
        Err(status) => match status {
            error::Status::NoDevice => {}
            _ => logln!("Error while registering secondary master: {}", status),
        },
        Ok(()) => {}
    }
    match filesystem::register_drive("/ide/secondary_slave") {
        Err(status) => match status {
            error::Status::NoDevice => {}
            _ => logln!("Error while registering secondary slave: {}", status),
        },
        Ok(()) => {}
    }
}
