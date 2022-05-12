use crate::{Header, Table, TablePointer};
use alloc::vec::Vec;
use core::mem::size_of;
use memory::KERNEL_VMA;

#[repr(packed(1))]
pub struct RSDT {
    header: Header,
    _tables: u32,
}

impl RSDT {
    pub fn get_tables(&self) -> Vec<TablePointer> {
        let mut ret = Vec::new();

        let length =
            ((self.header.length as usize - size_of::<Header>()) / size_of::<u32>()) as isize;
        let ptr = self as *const _ as *const u8;
        let ptr = unsafe { ptr.offset(size_of::<Header>() as isize) } as *const u32;
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

impl Table for RSDT {
    const SIGNATURE: &'static str = "RSDT";

    fn verify(&self) -> bool {
        self.header.calculate_checksum() == 0
    }
}
