use alloc::alloc::{alloc_zeroed, dealloc};
use core::{alloc::Layout, arch::global_asm};

pub(in crate::process) struct FloatingPointStorage(*mut u8);

const FLOATING_POINT_STORAGE_SIZE: usize = 512;
const FLOATING_POINT_STORAGE_LAYOUT: Layout =
    match Layout::from_size_align(FLOATING_POINT_STORAGE_SIZE, 16) {
        Ok(layout) => layout,
        Err(_) => panic!("Invalid floating point storage layout"),
    };

global_asm!(include_str!("float.asm"));

extern "C" {
    fn save_float(floating_point_storage: *mut u8);
    fn load_float(floating_point_storage: *const u8);
}

impl FloatingPointStorage {
    pub(in crate::process) fn new() -> Self {
        FloatingPointStorage(unsafe { alloc_zeroed(FLOATING_POINT_STORAGE_LAYOUT) })
    }

    pub(in crate::process) unsafe fn load_float(&self) {
        load_float(self.0)
    }

    pub(in crate::process) unsafe fn save_float(&self) {
        save_float(self.0)
    }
}

impl Drop for FloatingPointStorage {
    fn drop(&mut self) {
        unsafe { dealloc(self.0, FLOATING_POINT_STORAGE_LAYOUT) }
    }
}
