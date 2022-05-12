mod rsdt;
mod xsdt;

use crate::Header;
use alloc::vec::Vec;
use core::sync::atomic::{AtomicPtr, Ordering};

pub use rsdt::*;
pub use xsdt::*;

pub enum RootTable {
    RSDT(&'static RSDT),
    XSDT(&'static XSDT),
}

pub struct TablePointer(AtomicPtr<Header>);

impl RootTable {
    pub fn get_tables(&self) -> Vec<TablePointer> {
        match self {
            RootTable::RSDT(rsdt) => rsdt.get_tables(),
            RootTable::XSDT(xsdt) => xsdt.get_tables(),
        }
    }
}

impl TablePointer {
    pub fn new(table: *mut Header) -> Self {
        TablePointer(AtomicPtr::new(table))
    }

    pub fn get(&self) -> *const Header {
        self.0.load(Ordering::Acquire)
    }
}
