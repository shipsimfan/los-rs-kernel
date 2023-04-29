use crate::{log_info, CriticalLock, Logger};
use buddy::{num_pages_to_order, BuddyAllocator};
use core::{
    arch::global_asm,
    ffi::c_void,
    ptr::NonNull,
    sync::atomic::{AtomicBool, Ordering},
};
use page_tables::*;

// ISSUE: Assumes 48-bit physical address bus and no PML5
// TODO: Find correct physical address width and check for PML5.
//  Also increase range from default safe 64 TB to map the whole upper address space

mod address_space;
mod buddy;
mod constants;
mod global_alloc;
mod map;
mod page_tables;
mod physical_address;

macro_rules! mask {
    ($size: expr) => {
        !($size - 1)
    };
}

pub(self) use mask;

pub use address_space::AddressSpace;
pub use constants::*;
pub use map::*;
pub use physical_address::*;

//pub use slab::SlabAllocator;

pub struct MemoryManager {
    initialized: AtomicBool,
    logger: Logger,
    buddy_allocator: CriticalLock<BuddyAllocator>,
}

static MEMORY_MANAGER: MemoryManager = MemoryManager::null();

static KERNEL_ADDRESS_SPACE: AddressSpace = AddressSpace::null();
static IDENTITY_MAP: CriticalLock<[PDPT; IDENTITY_MAP_NUM_PDPTS]> =
    CriticalLock::new([PDPT::null(); IDENTITY_MAP_NUM_PDPTS]);

global_asm!(include_str!("memory.asm"));

extern "C" {
    static __KERNEL_TOP: c_void;
}

impl MemoryManager {
    pub(self) const fn null() -> Self {
        MemoryManager {
            initialized: AtomicBool::new(false),
            logger: Logger::from("Memory Manager"),
            buddy_allocator: CriticalLock::new(BuddyAllocator::new()),
        }
    }

    pub fn get() -> &'static MemoryManager {
        &MEMORY_MANAGER
    }

    pub(crate) fn initialize<M: MemoryMap>(
        &self,
        memory_map: M,
        framebuffer_memory: (usize, usize),
    ) {
        assert!(!self.initialized.swap(true, Ordering::AcqRel));

        log_info!(self.logger, "Initializing");

        // Setup the PML 4
        KERNEL_ADDRESS_SPACE.identity_map(&mut *IDENTITY_MAP.lock());
        unsafe { KERNEL_ADDRESS_SPACE.set_as_active() };

        // Free memory from the memory map
        let kernel_top = PhysicalAddress::new(unsafe { &__KERNEL_TOP });
        let framebuffer_bottom = framebuffer_memory.0 & PAGE_MASK;
        let framebuffer_top = framebuffer_memory.0 + framebuffer_memory.1;
        let (memory_map_bottom, memory_map_top) = memory_map.bottom_and_top();
        let memory_map_bottom = memory_map_bottom & PAGE_MASK;

        buddy::initialize(
            memory_map,
            &[
                (0, kernel_top.into_virtual::<u8>() as usize),
                (framebuffer_bottom, framebuffer_top),
                (memory_map_bottom, memory_map_top),
            ],
        )
    }

    pub fn kernel_address_space(&self) -> &AddressSpace {
        &KERNEL_ADDRESS_SPACE
    }

    pub fn allocate_pages(&self, num_pages: usize) -> NonNull<u8> {
        self.buddy_allocator
            .lock()
            .allocate(num_pages_to_order(num_pages))
    }

    pub fn free_pages(&self, ptr: NonNull<u8>, num_pages: usize) {
        self.buddy_allocator
            .lock()
            .free(ptr, num_pages_to_order(num_pages));
    }
}
