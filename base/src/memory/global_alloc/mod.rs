use crate::{MemoryManager, SlabAllocator};
use core::{
    alloc::{Allocator, GlobalAlloc},
    ptr::NonNull,
};

use super::PAGE_SIZE;

pub struct GlobalAllocator {
    slabs: [SlabAllocator; SLAB_SIZES.len()],
}

#[global_allocator]
static GLOBAL_ALLOCATOR: GlobalAllocator = GlobalAllocator::new();

const SLAB_SIZES: &[usize] = &[8, 16, 32, 64, 128, 256, 512, 1024, 2048];

impl GlobalAllocator {
    pub const fn new() -> Self {
        GlobalAllocator {
            slabs: [
                SlabAllocator::new(SLAB_SIZES[0], SLAB_SIZES[0]),
                SlabAllocator::new(SLAB_SIZES[1], SLAB_SIZES[1]),
                SlabAllocator::new(SLAB_SIZES[2], SLAB_SIZES[2]),
                SlabAllocator::new(SLAB_SIZES[3], SLAB_SIZES[3]),
                SlabAllocator::new(SLAB_SIZES[4], SLAB_SIZES[4]),
                SlabAllocator::new(SLAB_SIZES[5], SLAB_SIZES[5]),
                SlabAllocator::new(SLAB_SIZES[6], SLAB_SIZES[6]),
                SlabAllocator::new(SLAB_SIZES[7], SLAB_SIZES[7]),
                SlabAllocator::new(SLAB_SIZES[8], SLAB_SIZES[8]),
            ],
        }
    }
}

unsafe impl GlobalAlloc for GlobalAllocator {
    unsafe fn alloc(&self, layout: core::alloc::Layout) -> *mut u8 {
        for i in 0..SLAB_SIZES.len() {
            if SLAB_SIZES[i] >= layout.size() {
                return self.slabs[i].allocate(layout).unwrap().as_ptr() as *mut _;
            }
        }

        MemoryManager::get()
            .allocate_pages(layout.size().next_multiple_of(PAGE_SIZE) / PAGE_SIZE)
            .as_ptr()
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: core::alloc::Layout) {
        for i in 0..SLAB_SIZES.len() {
            if SLAB_SIZES[i] >= layout.size() {
                return self.slabs[i].deallocate(NonNull::new(ptr).unwrap(), layout);
            }
        }

        MemoryManager::get().free_pages(
            NonNull::new(ptr).unwrap(),
            layout.size().next_multiple_of(PAGE_SIZE) / PAGE_SIZE,
        )
    }
}
