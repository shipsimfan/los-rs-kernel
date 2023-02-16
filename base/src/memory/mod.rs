use crate::CriticalLock;
use core::sync::atomic::{AtomicBool, Ordering};
use physical::PhysicalMemoryManager;

mod constants;
mod physical;

pub use constants::*;
pub use physical::{MemoryDescriptor, MemoryMap, PhysicalAddress};

pub struct MemoryManager {
    physical: CriticalLock<PhysicalMemoryManager>,

    initialized: AtomicBool,
}

static MEMORY_MANAGER: MemoryManager = MemoryManager::null();

impl MemoryManager {
    pub(self) const fn null() -> Self {
        MemoryManager {
            physical: CriticalLock::new(PhysicalMemoryManager::null()),

            initialized: AtomicBool::new(false),
        }
    }

    pub fn get() -> &'static MemoryManager {
        &MEMORY_MANAGER
    }

    pub fn initialize<M: MemoryMap>(&self, memory_map: M) {
        assert!(!self.initialized.swap(true, Ordering::AcqRel));

        self.physical.lock().initialize(memory_map);
    }
}
