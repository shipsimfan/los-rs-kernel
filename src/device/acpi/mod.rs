use crate::{locks::Mutex, log, logln, memory::KERNEL_VMA};
use alloc::{
    string::{String, ToString},
    vec::Vec,
};
use core::ffi::c_void;

mod table;

pub type MADT = table::MADT;
pub type HPET = table::HPET;

static TABLES: Mutex<Vec<table::TablePointer>> = Mutex::new(Vec::new());

pub fn initialize(rsdp: *const c_void) -> Result<(), String> {
    log!("Initializing ACPI . . . ");

    let rsdp = if (rsdp as usize) < KERNEL_VMA {
        ((rsdp as usize) + KERNEL_VMA) as *const c_void
    } else {
        rsdp
    };

    let rsdp: &table::RSDP = table::from_ptr(rsdp as usize)?;
    let root_table = rsdp.get_root_table()?;
    let tables = root_table.get_tables();
    let mut lock = TABLES.lock();
    for t in tables {
        lock.push(t);
    }
    drop(lock);

    logln!("\x1B2A2]OK\x1B]!");

    Ok(())
}

pub fn get_table<T: table::Table>() -> Result<&'static T, String> {
    let signature = T::get_signature();
    let lock = TABLES.lock();
    let mut iter = lock.iter();
    while let Some(t) = iter.next() {
        if unsafe { (*t.get()).check_signature(signature) } {
            return Ok(table::from_ptr(t.get() as usize)?);
        }
    }

    Err("Unable to locate table".to_string())
}
