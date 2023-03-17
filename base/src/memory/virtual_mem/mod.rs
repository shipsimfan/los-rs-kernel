use core::ptr::NonNull;

use super::{physical::PhysicalMemoryManager, MemoryUsage, KERNEL_VMA};
use crate::{CriticalLock, MemoryMap, PhysicalAddress};

mod address_space;
mod page;
mod table;

pub use address_space::AddressSpace;

pub struct VirtualMemoryManager {
    physical_manager: CriticalLock<PhysicalMemoryManager>,

    kernel_address_space: CriticalLock<AddressSpace>,
}

fn physical_to_virtual<T>(physical: PhysicalAddress) -> Option<NonNull<T>> {
    if *physical == 0 {
        None
    } else {
        Some(unsafe { NonNull::new_unchecked((*physical + KERNEL_VMA) as *mut T) })
    }
}

// Assumes the ptr was made from a physical address
fn ptr_to_physical<T>(ptr: *mut T) -> PhysicalAddress {
    (ptr as usize - KERNEL_VMA).into()
}

impl VirtualMemoryManager {
    pub(super) const fn null() -> Self {
        VirtualMemoryManager {
            physical_manager: CriticalLock::new(PhysicalMemoryManager::null()),

            kernel_address_space: CriticalLock::new(unsafe { AddressSpace::null() }),
        }
    }

    pub(super) fn initialize<M: MemoryMap>(&self, memory_map: M, usage: &mut MemoryUsage) {
        let mut physical_manager = self.physical_manager.lock();
        physical_manager.initialize(memory_map, usage);

        self.kernel_address_space
            .lock()
            .initialize_kernel_pml4(&mut physical_manager);
    }

    pub(self) fn physical_manager(&self) -> &CriticalLock<PhysicalMemoryManager> {
        &self.physical_manager
    }
}
