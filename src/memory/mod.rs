mod heap;
mod physical;
mod virtual_mem;

use crate::bootloader;

pub type PhysicalAddress = usize;
pub type VirtualAddress = usize;
pub type AddressSpace = virtual_mem::AddressSpace;

pub const PAGE_SIZE: usize = 4096;
pub const KERNEL_VMA: usize = 0xFFFF800000000000;

pub unsafe fn initialize(
    mmap: *const bootloader::MemoryMap,
    gmode: *const bootloader::GraphicsMode,
) {
    physical::initialize(mmap);
    virtual_mem::initialize(mmap, gmode);
    heap::initialize();
}

pub fn get_free_memory() -> usize {
    physical::get_num_free_pages() * PAGE_SIZE
}

pub fn get_total_memory() -> usize {
    physical::get_num_total_pages() * PAGE_SIZE
}

pub fn map_virtual_memory(virtual_address: VirtualAddress, physical_address: PhysicalAddress) {
    virtual_mem::allocate(virtual_address, physical_address)
}
