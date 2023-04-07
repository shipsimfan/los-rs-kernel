#![no_std]

extern crate alloc;

use alloc::vec::Vec;
use base::{log_error, log_info, BootVideo, CriticalLock, LogOutput, Logger};
use core::ptr::NonNull;

mod interpreter;
mod namespace;
mod parser;
mod tables;

pub use tables::RSDP;

pub fn initialize<B: BootVideo>(rsdp: NonNull<RSDP>, boot_video: &CriticalLock<B>) {
    let logger = Logger::from("ACPI");
    log_info!(logger, "Loading tables");

    match tables::load(rsdp, &logger) {
        Ok(namespace) => {
            let node = namespace.get_node(&[*b"_SB_", *b"PCI0", *b"_PRT"]).unwrap();

            let prt = interpreter::method::execute(
                &namespace,
                node,
                Vec::new(),
                "PRT Interpreter".into(),
            )
            .unwrap();
            writeln!(boot_video, "PRT: {}", prt);

            #[cfg(feature = "namespace_logging")]
            write!(boot_video, "{}", namespace)
        }
        Err(error) => log_error!(logger, "{}", error),
    }
}
