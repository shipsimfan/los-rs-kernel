use core::mem::size_of;

use alloc::vec::Vec;
use memory::KERNEL_VMA;

use crate::{Header, Table, TablePointer};

#[repr(packed(1))]
pub struct XSDT {
    header: Header,
    _tables: u64,
}

impl XSDT {
    pub fn get_tables(&self) -> Vec<TablePointer> {
        let mut ret = Vec::new();

        let length =
            ((self.header.length as usize - size_of::<Header>()) / size_of::<u64>()) as isize;
        let ptr = self as *const _ as *const u8;
        let ptr = unsafe { ptr.offset(size_of::<Header>() as isize) } as *const u64;
        let mut i = 0;
        while i < length {
            let mut table = unsafe { *ptr.offset(i) } as usize;
            if table < KERNEL_VMA {
                table += KERNEL_VMA;
            }
            ret.push(TablePointer::new(table as *mut Header));
            i += 1;
        }

        ret
    }
}

impl Table for XSDT {
    const SIGNATURE: &'static str = "XSDT";

    fn verify(&self) -> bool {
        self.header.calculate_checksum() == 0
    }
}
