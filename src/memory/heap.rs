use crate::locks;
use core::alloc::{GlobalAlloc, Layout};

use super::KERNEL_VMA;

struct Heap;

#[global_allocator]
static HEAP: Heap = Heap;
static TOP: locks::Mutex<usize> = locks::Mutex::new(KERNEL_VMA as usize + 0x100000000000);

unsafe impl GlobalAlloc for Heap {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let mut top = TOP.lock();
        if *top % layout.align() != 0 {
            *top -= *top % layout.align();
            *top += layout.align();
        }

        let ret = *top as *mut u8;
        *top += layout.size();

        ret
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {}
}
