#![no_std]

extern crate alloc;

use alloc::string::ToString;
use base::{BootVideo, CriticalLock, LogOutput, Logger};
use core::ptr::NonNull;

mod interpreter;
mod namespace;
mod parser;
mod tables;

pub use tables::RSDP;

pub fn initialize<B: BootVideo>(rsdp: NonNull<RSDP>, boot_video: &CriticalLock<B>) {
    let logger = Logger::new("ACPI");
    logger.log(base::Level::Info, "Loading tables");

    match tables::load(rsdp, &logger) {
        Ok(namespace) => write!(boot_video, "{}", namespace),
        Err(error) => logger.log_owned(base::Level::Error, error.to_string()),
    }
}
