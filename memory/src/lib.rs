#![no_std]

use base::{critical::CriticalLock, log_info};

pub use heap::Heap;
pub use virtual_mem::{allocate, AddressSpace, MemoryExceptionHandler};

mod heap;
mod physical;
mod virtual_mem;

pub type PhysicalAddress = usize;
pub type VirtualAddress = usize;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct MemoryUsage {
    page_size: usize, // bytes
    available_pages: usize,
    free_pages: usize,
    kernel_heap_pages: usize,
    userspace_pages: usize,
    kernel_stack_usage: usize, // bytes
}

pub const PAGE_SIZE: usize = 4096;
pub const KERNEL_VMA: usize = 0xFFFF800000000000;

static mut MEMORY_INITIALIZED: bool = false;

static MEMORY_USAGE: CriticalLock<MemoryUsage> = CriticalLock::new(MemoryUsage {
    page_size: PAGE_SIZE,
    available_pages: 0,
    free_pages: 0,
    kernel_heap_pages: 0,
    userspace_pages: 0,
    kernel_stack_usage: 0,
});

const MODULE_NAME: &str = "Memory";

pub fn initialize(
    mmap: &base::bootloader::MemoryMap,
    gmode: &base::bootloader::GraphicsMode,
    null_access_exception_handler: MemoryExceptionHandler,
    invalid_access_exception_handler: MemoryExceptionHandler,
) {
    log_info!("Initializing . . . ");

    unsafe {
        assert!(!MEMORY_INITIALIZED);
        MEMORY_INITIALIZED = true;

        physical::initialize(mmap);
        virtual_mem::initialize(
            mmap,
            gmode,
            null_access_exception_handler,
            invalid_access_exception_handler,
        );
        heap::initialize();
    }

    log_info!("Initialized!");
}

pub fn get_memory_usage() -> MemoryUsage {
    *MEMORY_USAGE.lock()
}

pub fn map_virtual_memory(virtual_address: VirtualAddress, physical_address: PhysicalAddress) {
    virtual_mem::allocate(virtual_address, physical_address)
}

impl MemoryUsage {
    pub fn free_memory(&self) -> usize {
        self.free_pages * self.page_size
    }

    pub fn available_memory(&self) -> usize {
        self.available_pages * self.page_size
    }
}
