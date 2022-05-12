#![no_std]

use alloc::{boxed::Box, vec::Vec};
use base::log_info;
use core::{ffi::c_void, mem::ManuallyDrop, ptr::null};
use memory::KERNEL_VMA;
use process::{Mutex, ProcessTypes};

extern crate alloc;

mod pics;
mod tables;

pub use pics::{end_interrupt, end_irq};
pub use tables::*;

type TablesType<T> = &'static Mutex<Vec<TablePointer>, T>;

static mut ACPI_INITIALIZED: bool = false;
static mut TABLES_PTR: *const c_void = null();

const MODULE_NAME: &str = "ACPI";

fn tables<T: ProcessTypes + 'static>() -> TablesType<T> {
    unsafe { &*(TABLES_PTR as *const _) }
}

pub fn initialize<T: ProcessTypes + 'static>(rsdp: *const c_void) {
    unsafe {
        assert!(!ACPI_INITIALIZED);
        ACPI_INITIALIZED = true;
    }

    log_info!("Initializing . . .");

    // Convert RSDP to a virtual address if required
    let rsdp = if (rsdp as usize) < KERNEL_VMA {
        (rsdp as usize + KERNEL_VMA) as *const c_void
    } else {
        rsdp
    };

    // Get ACPI Tables
    let rsdp = tables::from_ptr::<RSDP>(rsdp as usize).unwrap();
    let root_table = rsdp.get_root_table().unwrap();
    let tables = ManuallyDrop::new(Box::new(Mutex::<_, T>::new(root_table.get_tables())));

    // Populate the global tables list
    unsafe {
        TABLES_PTR = tables.as_ref() as *const _ as *const _;
    }

    // Initialize I/O APIC
    pics::initialize_io_apic::<T>();

    log_info!("Initialized!");
}

pub fn get_table<T1: Table, T2: ProcessTypes + 'static>() -> Option<&'static T1> {
    let lock = tables::<T2>().lock();
    let mut iter = lock.iter();
    while let Some(t) = iter.next() {
        if unsafe { (*t.get()).check_signature(T1::SIGNATURE) } {
            return Some(from_ptr(t.get() as usize)?);
        }
    }

    None
}
