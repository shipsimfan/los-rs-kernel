use crate::CriticalLock;
use core::sync::atomic::{AtomicBool, Ordering};
use physical::PhysicalMemoryManager;

mod constants;
mod physical;

pub use constants::*;

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

    pub fn initialize(&self, memory_map: *const uefi::memory::raw::MemoryMap) {
        assert!(!self.initialized.swap(true, Ordering::AcqRel));

        self.physical.lock().initialize(memory_map);
    }
}
