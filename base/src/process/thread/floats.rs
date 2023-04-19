use alloc::alloc::{alloc_zeroed, dealloc};
use core::alloc::Layout;

pub(in crate::process) struct FloatingPointStorage(*mut u8);

const FLOATING_POINT_STORAGE_SIZE: usize = 512;
const FLOATING_POINT_STORAGE_LAYOUT: Layout =
    match Layout::from_size_align(FLOATING_POINT_STORAGE_SIZE, 16) {
        Ok(layout) => layout,
        Err(_) => panic!("Invalid floating point storage layout"),
    };

impl FloatingPointStorage {
    pub(super) fn new() -> Self {
        FloatingPointStorage(unsafe { alloc_zeroed(FLOATING_POINT_STORAGE_LAYOUT) })
    }
}

impl Drop for FloatingPointStorage {
    fn drop(&mut self) {
        unsafe { dealloc(self.0, FLOATING_POINT_STORAGE_LAYOUT) }
    }
}
