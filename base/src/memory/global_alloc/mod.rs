use crate::{memory::PAGE_SIZE, MemoryManager};
use core::{alloc::GlobalAlloc, ptr::NonNull};

pub struct GlobalAllocator {
    //slabs: [SlabAllocator; SLAB_SIZES.len()],
}

#[global_allocator]
static GLOBAL_ALLOCATOR: GlobalAllocator = GlobalAllocator::new();

const SLAB_SIZES: &[usize] = &[8, 16, 32, 64, 128, 256, 512, 1024, LARGEST_SLAB_SIZE];
const LARGEST_SLAB_SIZE: usize = 2048;

impl GlobalAllocator {
    pub const fn new() -> Self {
        GlobalAllocator {
            /*slabs: [
                SlabAllocator::new(SLAB_SIZES[0], SLAB_SIZES[0]),
                SlabAllocator::new(SLAB_SIZES[1], SLAB_SIZES[1]),
                SlabAllocator::new(SLAB_SIZES[2], SLAB_SIZES[2]),
                SlabAllocator::new(SLAB_SIZES[3], SLAB_SIZES[3]),
                SlabAllocator::new(SLAB_SIZES[4], SLAB_SIZES[4]),
                SlabAllocator::new(SLAB_SIZES[5], SLAB_SIZES[5]),
                SlabAllocator::new(SLAB_SIZES[6], SLAB_SIZES[6]),
                SlabAllocator::new(SLAB_SIZES[7], SLAB_SIZES[7]),
                SlabAllocator::new(SLAB_SIZES[8], SLAB_SIZES[8]),
            ],*/
        }
    }
}

unsafe impl GlobalAlloc for GlobalAllocator {
    unsafe fn alloc(&self, layout: core::alloc::Layout) -> *mut u8 {
        if layout.size() > LARGEST_SLAB_SIZE {
            return MemoryManager::get()
                .allocate_pages((layout.size() + PAGE_SIZE - 1) / PAGE_SIZE)
                .as_ptr();
        }

        todo!();

        /*
        for i in 0..SLAB_SIZES.len() {
            if SLAB_SIZES[i] >= layout.size() {
                return self.slabs[i].allocate(layout).unwrap().as_ptr() as *mut _;
            }
        }

        MemoryManager::get()
            .allocate_pages(layout.size().next_multiple_of(PAGE_SIZE) / PAGE_SIZE)
            .as_ptr()
        */
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: core::alloc::Layout) {
        if layout.size() > LARGEST_SLAB_SIZE {
            return MemoryManager::get().free_pages(
                NonNull::new(ptr).unwrap(),
                (layout.size() + PAGE_SIZE - 1) / PAGE_SIZE,
            );
        }

        todo!();

        /*
        for i in 0..SLAB_SIZES.len() {
            if SLAB_SIZES[i] >= layout.size() {
                return self.slabs[i].deallocate(NonNull::new(ptr).unwrap(), layout);
            }
        }

        MemoryManager::get().free_pages(
            NonNull::new(ptr).unwrap(),
            layout.size().next_multiple_of(PAGE_SIZE) / PAGE_SIZE,
        )
        */
    }
}
