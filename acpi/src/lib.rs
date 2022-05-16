#![no_std]

use base::log_info;
use core::ffi::c_void;
use memory::KERNEL_VMA;
use process::{Mutex, ProcessTypes};

extern crate alloc;

mod pics;
mod tables;

pub use pics::{end_interrupt, end_irq};
pub use tables::*;

static mut ACPI_INITIALIZED: bool = false;

process::static_generic!(
    process::Mutex<alloc::vec::Vec<crate::tables::TablePointer>, T>,
    loaded_tables
);

const MODULE_NAME: &str = "ACPI";

pub fn initialize<T: ProcessTypes + 'static>(rsdp: *const c_void) {
    log_info!("Initializing . . .");

    unsafe {
        assert!(!ACPI_INITIALIZED);
        ACPI_INITIALIZED = true;
    }

    // Convert RSDP to a virtual address if required
    let rsdp = if (rsdp as usize) < KERNEL_VMA {
        (rsdp as usize + KERNEL_VMA) as *const c_void
    } else {
        rsdp
    };

    // Get ACPI Tables
    let rsdp = tables::from_ptr::<RSDP>(rsdp as usize).unwrap();
    let root_table = rsdp.get_root_table().unwrap();
    loaded_tables::initialize::<T>(Mutex::new(root_table.get_tables()));

    // Initialize I/O APIC
    pics::initialize_io_apic::<T>();

    log_info!("Initialized!");
}

pub fn get_table<T1: Table, T2: ProcessTypes + 'static>() -> Option<&'static T1> {
    let lock = loaded_tables::get::<T2>().lock();
    let mut iter = lock.iter();
    while let Some(t) = iter.next() {
        if unsafe { (*t.get()).check_signature(T1::SIGNATURE) } {
            return Some(from_ptr(t.get() as usize)?);
        }
    }

    None
}
