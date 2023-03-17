use super::{
    ptr_to_physical,
    table::{PDPT, PML4},
};
use crate::{
    memory::{physical::PhysicalMemoryManager, KERNEL_VMA, PAGE_SIZE, TABLE_ENTRIES},
    PhysicalAddress,
};
use core::ptr::{null_mut, NonNull};

pub struct AddressSpace(NonNull<PML4>);

impl AddressSpace {
    pub(super) const unsafe fn null() -> Self {
        AddressSpace(NonNull::new_unchecked(null_mut()))
    }

    pub(super) fn new(physical_manager: &mut PhysicalMemoryManager, kernel_pml4: &PML4) -> Self {
        let mut new_pml4 = PML4::new(physical_manager);

        for i in TABLE_ENTRIES / 2..TABLE_ENTRIES {
            unsafe { new_pml4.as_mut().clone_entry(i, kernel_pml4) };
        }

        AddressSpace(new_pml4)
    }

    pub(super) fn initialize_kernel_pml4(&mut self, physical_manager: &mut PhysicalMemoryManager) {
        self.0 = PML4::new(physical_manager);

        // Create PDPTs for the upper half
        for i in TABLE_ENTRIES / 2..TABLE_ENTRIES {
            let pdpt = PDPT::new(physical_manager);
            unsafe { self.0.as_mut().set_entry(i, pdpt.as_ptr(), false, true) };
        }

        // Map all of physical memory
        let total_pages = *physical_manager.top() / PAGE_SIZE;
        let mut address = KERNEL_VMA;

        todo!();
    }

    pub(in super::super) fn allocate(start: usize, num_pages: usize, user: bool, write: bool) {
        todo!()
    }

    pub(in super::super) fn free(start: usize, num_pages: usize) {
        todo!()
    }

    fn get_physical(&self) -> PhysicalAddress {
        ptr_to_physical(self.0.as_ptr())
    }
}
