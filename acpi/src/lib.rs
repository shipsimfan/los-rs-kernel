#![no_std]

use base::Logger;
use core::ptr::NonNull;
use tables::Table;

mod tables;

pub use tables::RSDP;

pub fn initialize(rsdp: NonNull<RSDP>) {
    let logger = Logger::new("ACPI");
    logger.log(base::Level::Info, "Initializing");

    let rsdp = unsafe { rsdp.as_ref() };
    if !rsdp.verify() {
        logger.log(base::Level::Error, "Invalid RSDP");
        return;
    }

    let xsdt = match rsdp.xsdt() {
        Some(xsdt) => unsafe { xsdt.as_ref() },
        None => {
            logger.log(base::Level::Error, "No XSDT in RSDP");
            return;
        }
    };
    if !xsdt.verify() {
        logger.log(base::Level::Error, "Invalid XSDT");
        return;
    }

    logger.log(base::Level::Debug, "XSDT Verified");
}
