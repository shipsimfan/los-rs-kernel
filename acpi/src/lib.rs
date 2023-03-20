#![no_std]

extern crate alloc;

use alloc::format;
use aml::AML;
use base::{BootVideo, CriticalLock, LogOutput, Logger};
use core::ptr::NonNull;
use tables::{DSDT, FADT, XSDT};

mod aml;
mod tables;

pub use tables::RSDP;

macro_rules! unwrap_or_return {
    ($expr: expr) => {
        match $expr {
            Some(inner) => inner,
            None => return,
        }
    };
}

pub fn initialize<B: BootVideo>(rsdp: NonNull<RSDP>, boot_video: &CriticalLock<B>) {
    let logger = Logger::new("ACPI");
    logger.log(base::Level::Info, "Initializing");

    let rsdp = unwrap_or_return!(RSDP::get(rsdp, &logger));

    let xsdt = unwrap_or_return!(XSDT::get(unwrap_or_return!(rsdp.xsdt(&logger)), &logger));

    let fadt = match xsdt.get_table::<tables::FADT>() {
        Some(fadt) => unwrap_or_return!(FADT::get(fadt, &logger)),
        None => {
            logger.log(base::Level::Error, "No FADT in XSDT");
            return;
        }
    };

    let dsdt = unwrap_or_return!(DSDT::get(fadt.dsdt(), &logger));

    logger.log(base::Level::Info, "Parsing DSDT");
    let aml = match AML::parse(dsdt.definition_block()) {
        Ok(aml) => aml,
        Err(error) => {
            logger.log_owned(base::Level::Error, format!("Error: {}", error));
            return;
        }
    };

    write!(boot_video, "{}", aml);
}
