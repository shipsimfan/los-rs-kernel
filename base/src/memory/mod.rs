use crate::{CriticalLock, Logger};
use buddy::BuddyAllocator;
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

mod buddy;
mod constants;
mod global_alloc;
mod map;
mod page_tables;
mod physical_address;
mod slab;

macro_rules! mask {
    ($size: expr) => {
        !($size - 1)
    };
}

pub(self) use mask;

pub use constants::*;
pub use map::*;
pub use physical_address::*;
pub use slab::SlabAllocator;

pub struct MemoryManager {
    initialized: AtomicBool,
    logger: Logger,

    buddy_allocator: CriticalLock<BuddyAllocator>,
}

static MEMORY_MANAGER: MemoryManager = MemoryManager::null();

static mut KERNEL_PML4: PML4 = PML4::null();
static mut IDENTITY_MAP: [PDPT; IDENTITY_MAP_NUM_PDPTS] = [PDPT::null(); IDENTITY_MAP_NUM_PDPTS];

global_asm!(include_str!("memory.asm"));

extern "C" {
    static __KERNEL_TOP: c_void;

    fn set_cr3(cr3: usize);
}

impl MemoryManager {
    pub(self) const fn null() -> Self {
        MemoryManager {
            initialized: AtomicBool::new(false),
            logger: Logger::new("Memory Manager"),

            buddy_allocator: CriticalLock::new(BuddyAllocator::new()),
        }
    }

    pub fn get() -> &'static MemoryManager {
        &MEMORY_MANAGER
    }

    pub fn initialize<M: MemoryMap>(&self, memory_map: M, framebuffer_memory: (usize, usize)) {
        assert!(!self.initialized.swap(true, Ordering::AcqRel));

        self.logger.log(crate::Level::Info, "Initializing");

        // Setup the PML 4
        unsafe {
            KERNEL_PML4.identity_map(&mut IDENTITY_MAP);
            set_cr3(PhysicalAddress::new(&KERNEL_PML4).into_usize());
        }

        // Free memory from the memory map
        let kernel_top = PhysicalAddress::new(unsafe { &__KERNEL_TOP });
        let framebuffer_bottom = framebuffer_memory.0 & PAGE_MASK;
        let framebuffer_top = framebuffer_memory.0 + framebuffer_memory.1;
        let (memory_map_bottom, memory_map_top) = memory_map.bottom_and_top();
        let memory_map_bottom = memory_map_bottom & PAGE_MASK;

        let mut buddy_allocator = self.buddy_allocator.lock();
        unsafe { buddy_allocator.initialize_orders() };
        for descriptor in memory_map {
            if !descriptor.is_usable() {
                continue;
            }

            let mut address = descriptor.address();
            if unsafe { address.add(descriptor.num_pages() * PAGE_SIZE) } <= kernel_top {
                continue;
            }

            let mut virt_bottom = address.into_virtual::<u32>() as usize;
            let virt_top = virt_bottom + (descriptor.num_pages() * PAGE_SIZE);
            if (virt_bottom >= framebuffer_bottom && virt_top <= framebuffer_top)
                || (virt_bottom >= memory_map_bottom && virt_top <= memory_map_top)
            {
                continue;
            }

            for _ in 0..descriptor.num_pages() {
                if address >= kernel_top
                    && (virt_bottom <= framebuffer_bottom - PAGE_SIZE
                        || virt_bottom >= framebuffer_top)
                    && (virt_bottom <= memory_map_bottom - PAGE_SIZE
                        || virt_bottom >= memory_map_top)
                {
                    unsafe { buddy_allocator.free_at(address.into_virtual::<()>() as usize, 1) };
                }

                virt_bottom += PAGE_SIZE;
                address = unsafe { address.add(PAGE_SIZE) };
            }
        }
    }

    pub fn allocate_pages(&self, num_pages: usize) -> NonNull<u8> {
        unsafe { NonNull::new_unchecked(self.buddy_allocator.lock().allocate(num_pages) as *mut _) }
    }

    pub fn free_pages(&self, ptr: NonNull<u8>, num_pages: usize) {
        self.buddy_allocator
            .lock()
            .free(ptr.as_ptr() as usize, num_pages);
    }
}
