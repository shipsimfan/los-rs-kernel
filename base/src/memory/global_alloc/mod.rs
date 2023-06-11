use super::slab::Cache;
use crate::{memory::PAGE_SIZE, CriticalLock, MemoryManager};
use core::{alloc::GlobalAlloc, ptr::NonNull};

pub struct GlobalAllocator {
    size_caches: [CriticalLock<Cache>; CACHE_SIZES.len()],
}

#[global_allocator]
static GLOBAL_ALLOCATOR: GlobalAllocator = GlobalAllocator::new();

const CACHE_SIZES: &[usize] = &[8, 16, 32, 64, 128, 256, 512, 1024, LARGEST_SLAB_SIZE];
const LARGEST_SLAB_SIZE: usize = 2048;

impl GlobalAllocator {
    pub const fn new() -> Self {
        GlobalAllocator {
            size_caches: [
                CriticalLock::new(Cache::new(CACHE_SIZES[0], CACHE_SIZES[0])),
                CriticalLock::new(Cache::new(CACHE_SIZES[1], CACHE_SIZES[1])),
                CriticalLock::new(Cache::new(CACHE_SIZES[2], CACHE_SIZES[2])),
                CriticalLock::new(Cache::new(CACHE_SIZES[3], CACHE_SIZES[3])),
                CriticalLock::new(Cache::new(CACHE_SIZES[4], CACHE_SIZES[4])),
                CriticalLock::new(Cache::new(CACHE_SIZES[5], CACHE_SIZES[5])),
                CriticalLock::new(Cache::new(CACHE_SIZES[6], CACHE_SIZES[6])),
                CriticalLock::new(Cache::new(CACHE_SIZES[7], CACHE_SIZES[7])),
                CriticalLock::new(Cache::new(CACHE_SIZES[8], CACHE_SIZES[8])),
            ],
        }
    }
}

unsafe impl GlobalAlloc for GlobalAllocator {
    unsafe fn alloc(&self, layout: core::alloc::Layout) -> *mut u8 {
        for i in 0..CACHE_SIZES.len() {
            if CACHE_SIZES[i] >= layout.size() {
                return self.size_caches[i].lock().allocate().as_ptr();
            }
        }

        MemoryManager::get()
            .allocate_pages(layout.size().next_multiple_of(PAGE_SIZE) / PAGE_SIZE)
            .as_ptr()
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: core::alloc::Layout) {
        let ptr = NonNull::new(ptr).unwrap();

        for i in 0..CACHE_SIZES.len() {
            if CACHE_SIZES[i] >= layout.size() {
                return self.size_caches[i].lock().free(ptr);
            }
        }

        MemoryManager::get().free_pages(ptr, layout.size().next_multiple_of(PAGE_SIZE) / PAGE_SIZE)
    }
}
