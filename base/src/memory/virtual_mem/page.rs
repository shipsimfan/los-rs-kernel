use super::{ptr_to_physical, table::PhysicalDrop};
use crate::memory::{physical::PhysicalMemoryManager, MemoryUsage, KERNEL_VMA, PAGE_SIZE};

pub(super) type Page = [u8; PAGE_SIZE];

impl PhysicalDrop for Page {
    fn physical_drop(
        &mut self,
        physical_manager: &mut PhysicalMemoryManager,
        usage: &mut MemoryUsage,
    ) {
        let address = ptr_to_physical(self.as_mut_ptr());
        for i in self {
            *i = 0;
        }

        if *address < KERNEL_VMA {
            usage.free_userspace_page();
        } /* TODO: else if addr >= KERNEL_VMA + super::heap::HEAP_START_OFFSET {
              usage.free_kernel_heap_page()
          } */

        physical_manager.free(address);
    }
}
