use super::PAGE_SIZE;
use bitmap::Bitmap;

mod address;
mod bitmap;

pub(super) use address::PhysicalAddress;

pub(super) struct PhysicalMemoryManager {
    page_bitmap: Bitmap<MAXIMUM_PHYSICAL_PAGES>,
}

const MAXIMUM_PHYSICAL_MEMORY: usize = 256 * (1024 * 1024 * 1024); // 256 Gigabytes
const MAXIMUM_PHYSICAL_PAGES: usize = MAXIMUM_PHYSICAL_MEMORY / PAGE_SIZE;

impl PhysicalMemoryManager {
    pub(super) const fn null() -> Self {
        PhysicalMemoryManager {
            page_bitmap: Bitmap::null(),
        }
    }

    pub(super) fn initialize(&mut self, memory_map: *const uefi::memory::raw::MemoryMap) {}

    pub(super) fn allocate(&mut self) -> PhysicalAddress {
        panic!("TODO: Implement")
    }

    pub(super) fn free(&mut self, address: PhysicalAddress) {}
}
