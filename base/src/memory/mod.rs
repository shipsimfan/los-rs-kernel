use crate::CriticalLock;
use core::sync::atomic::{AtomicBool, Ordering};
use page_tables::*;

// ISSUE: Assumes 48-bit physical address bus and no PML5
// TODO: Find correct physical address width and check for PML5.
//  Also increase range from default safe 64 TB to map the whole upper address space

mod constants;
mod map;
mod page_tables;
mod physical_address;

pub use constants::*;
pub use map::*;
pub use physical_address::*;

pub struct MemoryManager {
    initialized: AtomicBool,

    kernel_pml4: CriticalLock<PML4>,
    identity_mapping: CriticalLock<[PDPT; IDENTITY_MAP_NUM_PDPTS]>,
}

static MEMORY_MANAGER: MemoryManager = MemoryManager::null();

impl MemoryManager {
    pub(self) const fn null() -> Self {
        MemoryManager {
            initialized: AtomicBool::new(false),

            kernel_pml4: CriticalLock::new(PML4::null()),
            identity_mapping: CriticalLock::new([PDPT::null(); IDENTITY_MAP_NUM_PDPTS]),
        }
    }

    pub fn get() -> &'static MemoryManager {
        &MEMORY_MANAGER
    }

    pub fn initialize<M: MemoryMap>(&self, memory_map: M) {
        assert!(!self.initialized.swap(true, Ordering::AcqRel));

        self.kernel_pml4
            .lock()
            .identity_map(&mut *self.identity_mapping.lock());

        // TODO: Setup the buddy allocator and free memory from the memory map
    }
}
