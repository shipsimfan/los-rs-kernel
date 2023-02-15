use self::physical::PhysicalMemoryManager;
use crate::CriticalLock;

mod constants;
mod physical;

pub use constants::*;

pub struct MemoryManager {
    physical: PhysicalMemoryManager,
}

#[allow(unused)]
static MEMORY_MANAGER: CriticalLock<MemoryManager> = CriticalLock::new(MemoryManager::null());

impl MemoryManager {
    pub(self) const fn null() -> Self {
        MemoryManager {
            physical: PhysicalMemoryManager::null(),
        }
    }
}
