use super::{clear_bit, set_bit, PageDirectory};
use crate::{
    memory::{IDENTITY_MAP_PAGE_SIZE, PAGE_MASK, TABLE_ENTRIES},
    PhysicalAddress,
};

#[repr(align(4096))]
#[derive(Clone, Copy)]
pub(in crate::memory) struct PDPT([PDPTEntry; TABLE_ENTRIES]);

#[derive(Clone, Copy)]
pub(in crate::memory) struct PDPTEntry(usize);

const PRESENT_BIT: u32 = 0;
const READ_WRITE_BIT: u32 = 1;
const USER_SUPERVISOR_BIT: u32 = 2;
const PAGE_SIZE_BIT: u32 = 7;

impl PDPT {
    pub(in crate::memory) const fn null() -> Self {
        PDPT([PDPTEntry::null(); TABLE_ENTRIES])
    }

    pub(super) fn identity_map(&mut self, base_address: PhysicalAddress) {
        let mut address = base_address;
        for i in 0..TABLE_ENTRIES {
            let mut entry = PDPTEntry::from_address(address);
            entry.set_present();
            entry.set_write();
            entry.set_supervisor();

            self.set_entry(i, entry);

            address = unsafe { address.add(IDENTITY_MAP_PAGE_SIZE) };
        }
    }

    pub fn set_entry(&mut self, index: usize, entry: PDPTEntry) {
        if index == 0 {
            // Test
        }

        self.0[index] = entry;
    }
}

impl PDPTEntry {
    pub(self) const fn null() -> Self {
        PDPTEntry(0)
    }

    pub fn from_address(address: PhysicalAddress) -> Self {
        let mut entry = unsafe { address.into_usize() } & PAGE_MASK;

        set_bit!(entry, PAGE_SIZE_BIT);

        PDPTEntry(entry)
    }

    #[allow(unused)]
    pub fn new(page_directory: &PageDirectory) -> Self {
        let mut address = unsafe { PhysicalAddress::new(page_directory).into_usize() } & PAGE_MASK;

        clear_bit!(address, PAGE_SIZE_BIT);

        PDPTEntry(address)
    }

    #[allow(unused)]
    pub fn clear_present(&mut self) {
        clear_bit!(self.0, PRESENT_BIT);
    }

    pub fn set_present(&mut self) {
        set_bit!(self.0, PRESENT_BIT);
    }

    #[allow(unused)]
    pub fn set_read(&mut self) {
        clear_bit!(self.0, READ_WRITE_BIT);
    }

    pub fn set_supervisor(&mut self) {
        clear_bit!(self.0, USER_SUPERVISOR_BIT);
    }

    #[allow(unused)]
    pub fn set_user(&mut self) {
        set_bit!(self.0, USER_SUPERVISOR_BIT);
    }

    pub fn set_write(&mut self) {
        set_bit!(self.0, READ_WRITE_BIT);
    }
}
