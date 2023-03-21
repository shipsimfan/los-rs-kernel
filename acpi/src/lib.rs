#![no_std]

extern crate alloc;

use alloc::string::ToString;
use base::{BootVideo, CriticalLock, LogOutput, Logger};
use core::ptr::NonNull;
use namespace::Namespace;

mod aml;
mod loader;
mod namespace;
mod tables;

pub use tables::RSDP;

pub fn initialize<B: BootVideo>(rsdp: NonNull<RSDP>, boot_video: &CriticalLock<B>) {
    let logger = Logger::new("ACPI");
    logger.log(base::Level::Info, "Loading tables");

    let mut namespace = Namespace::new();
    match loader::load(rsdp, &mut namespace) {
        Ok(()) => {
            write!(boot_video, "{}", namespace)
        }
        Err(error) => logger.log_owned(base::Level::Error, error.to_string()),
    }
}
