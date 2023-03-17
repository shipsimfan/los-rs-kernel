use core::sync::atomic::{AtomicBool, Ordering};
use virtual_mem::VirtualMemoryManager;

mod constants;
mod physical;
mod usage;
mod virtual_mem;

pub use constants::*;
pub use physical::{MemoryDescriptor, MemoryMap, PhysicalAddress};
pub use usage::MemoryUsage;
pub use virtual_mem::AddressSpace;

use crate::CriticalLock;

pub struct MemoryManager {
    initialized: AtomicBool,

    virtual_manager: VirtualMemoryManager,
    usage: CriticalLock<MemoryUsage>,
}

static MEMORY_MANAGER: MemoryManager = MemoryManager::null();

impl MemoryManager {
    pub(self) const fn null() -> Self {
        MemoryManager {
            initialized: AtomicBool::new(false),

            virtual_manager: VirtualMemoryManager::null(),
            usage: CriticalLock::new(MemoryUsage::null()),
        }
    }

    pub fn get() -> &'static MemoryManager {
        &MEMORY_MANAGER
    }

    pub fn initialize<M: MemoryMap>(&self, memory_map: M) {
        assert!(!self.initialized.swap(true, Ordering::AcqRel));

        self.virtual_manager
            .initialize(memory_map, &mut self.usage.lock());
    }

    pub fn virtual_manager(&self) -> &VirtualMemoryManager {
        &self.virtual_manager
    }

    pub fn usage(&self) -> &CriticalLock<MemoryUsage> {
        &self.usage
    }
}
