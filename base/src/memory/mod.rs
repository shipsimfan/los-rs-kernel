use core::{
    arch::global_asm,
    sync::atomic::{AtomicBool, Ordering},
};
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
}

static MEMORY_MANAGER: MemoryManager = MemoryManager::null();

static mut KERNEL_PML4: PML4 = PML4::null();
static mut IDENTITY_MAP: [PDPT; IDENTITY_MAP_NUM_PDPTS] = [PDPT::null(); IDENTITY_MAP_NUM_PDPTS];

global_asm!(include_str!("memory.asm"));

extern "C" {
    fn set_cr3(cr3: usize);
}

impl MemoryManager {
    pub(self) const fn null() -> Self {
        MemoryManager {
            initialized: AtomicBool::new(false),
        }
    }

    pub fn get() -> &'static MemoryManager {
        &MEMORY_MANAGER
    }

    pub fn initialize<M: MemoryMap>(&self, memory_map: M) {
        assert!(!self.initialized.swap(true, Ordering::AcqRel));

        // Setup the PML 4
        unsafe {
            KERNEL_PML4.identity_map(&mut IDENTITY_MAP);
            set_cr3(PhysicalAddress::new(&KERNEL_PML4).into_usize());
        }

        // TODO: Setup the buddy allocator and free memory from the memory map
    }
}
