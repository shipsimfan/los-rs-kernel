use core::alloc::{GlobalAlloc, Layout};

use super::KERNEL_VMA;

struct Heap;

#[global_allocator]
static HEAP: Heap = Heap;
static mut TOP: usize = KERNEL_VMA as usize + 0x100000000000;

unsafe impl GlobalAlloc for Heap {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        if TOP % layout.align() != 0 {
            TOP -= TOP % layout.align();
            TOP += layout.align();
        }

        let ret = TOP as *mut u8;
        TOP += layout.size();

        ret
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {}
}
