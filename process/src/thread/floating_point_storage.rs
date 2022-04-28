pub struct FloatingPointStorage {
    ptr: *mut u8,
}

const FLOATING_POINT_STORAGE_SIZE: usize = 512;
const FLOATING_POINT_STORAGE_LAYOUT: alloc::alloc::Layout =
    unsafe { alloc::alloc::Layout::from_size_align_unchecked(FLOATING_POINT_STORAGE_SIZE, 16) };

impl FloatingPointStorage {
    pub fn new() -> Self {
        FloatingPointStorage {
            ptr: unsafe { alloc::alloc::alloc_zeroed(FLOATING_POINT_STORAGE_LAYOUT) },
        }
    }

    pub fn as_mut_ptr(&mut self) -> *mut u8 {
        self.ptr
    }
}

impl Drop for FloatingPointStorage {
    fn drop(&mut self) {
        unsafe { alloc::alloc::dealloc(self.ptr, FLOATING_POINT_STORAGE_LAYOUT) };
    }
}
