use core::ops::Index;

use super::{clear_bit, set_bit, PDPT};
use crate::{
    memory::{IDENTITY_MAP_PAGE_SIZE, PAGE_MASK, TABLE_ENTRIES},
    PhysicalAddress,
};

#[repr(align(4096))]
pub(in crate::memory) struct PML4([PML4Entry; TABLE_ENTRIES]);

#[derive(Clone, Copy)]
pub(in crate::memory) struct PML4Entry(usize);

const PRESENT_BIT: u32 = 0;
const READ_WRITE_BIT: u32 = 1;
const USER_SUPERVISOR_BIT: u32 = 2;

impl PML4 {
    pub(in crate::memory) const fn null() -> Self {
        PML4([PML4Entry::null(); TABLE_ENTRIES])
    }

    pub(in crate::memory) fn as_ptr(&self) -> *const PML4 {
        self
    }

    pub(in crate::memory) fn identity_map(&mut self, pdpts: &mut [PDPT]) {
        let mut address = unsafe { PhysicalAddress::from_raw(0) };

        for i in 0..pdpts.len() {
            pdpts[i].identity_map(address);

            let mut entry = PML4Entry::new(&pdpts[i]);
            entry.set_present();
            entry.set_write();
            entry.set_supervisor();

            self.set_entry(i + TABLE_ENTRIES / 2, entry);

            address = unsafe { address.add(IDENTITY_MAP_PAGE_SIZE * TABLE_ENTRIES) };
        }
    }

    pub(in crate::memory) fn set_entry(&mut self, index: usize, entry: PML4Entry) {
        self.0[index] = entry;
    }
}

impl Index<usize> for PML4 {
    type Output = PML4Entry;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl PML4Entry {
    pub(self) const fn null() -> Self {
        PML4Entry(0)
    }

    pub fn new(pdpt: &PDPT) -> Self {
        let address = PhysicalAddress::new(pdpt);

        PML4Entry(unsafe { address.into_usize() } & PAGE_MASK)
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
