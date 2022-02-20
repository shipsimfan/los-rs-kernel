mod heap;
mod physical;
mod virtual_mem;

use crate::{bootloader, critical::CriticalLock};

pub type PhysicalAddress = usize;
pub type VirtualAddress = usize;
pub type AddressSpace = virtual_mem::AddressSpace;

pub const PAGE_SIZE: usize = 4096;
pub const KERNEL_VMA: usize = 0xFFFF800000000000;

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

static MEMORY_USAGE: CriticalLock<MemoryUsage> = CriticalLock::new(MemoryUsage {
    page_size: PAGE_SIZE,
    available_pages: 0,
    free_pages: 0,
    kernel_heap_pages: 0,
    userspace_pages: 0,
    kernel_stack_usage: 0,
});

pub unsafe fn initialize(
    mmap: *const bootloader::MemoryMap,
    gmode: *const bootloader::GraphicsMode,
) {
    physical::initialize(mmap);
    virtual_mem::initialize(mmap, gmode);
    heap::initialize();
}

pub fn get_memory_usage() -> MemoryUsage {
    *MEMORY_USAGE.lock()
}

pub fn allocate_kernel_stack(size: usize) {
    MEMORY_USAGE.lock().kernel_stack_usage += size;
}

pub fn free_kernel_stack(size: usize) {
    MEMORY_USAGE.lock().kernel_stack_usage -= size;
}

pub fn map_virtual_memory(virtual_address: VirtualAddress, physical_address: PhysicalAddress) {
    virtual_mem::allocate(virtual_address, physical_address)
}

impl MemoryUsage {
    pub fn free_memory(&self) -> usize {
        self.page_size * self.free_pages
    }

    pub fn available_memory(&self) -> usize {
        self.page_size * self.available_pages
    }
}
