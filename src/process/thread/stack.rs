use crate::memory;

pub struct Stack {
    stack: &'static mut [u8],
    top: usize,
    pointer: usize,
}

const KERNEL_STACK_SIZE: usize = 32 * 1024; // 32 Kilobytes
const KERNEL_STACK_LAYOUT: alloc::alloc::Layout =
    unsafe { alloc::alloc::Layout::from_size_align_unchecked(KERNEL_STACK_SIZE, 16) };

impl Stack {
    pub fn new() -> Self {
        let stack = unsafe { alloc::alloc::alloc_zeroed(KERNEL_STACK_LAYOUT) };
        let stack = unsafe { core::slice::from_raw_parts_mut(stack, KERNEL_STACK_LAYOUT.size()) };

        memory::allocate_kernel_stack(KERNEL_STACK_SIZE);

        // Set the kernel stack pointer appropriately
        let top = stack.as_ptr() as usize + KERNEL_STACK_SIZE;
        Stack {
            stack: stack,
            top: top,
            pointer: top,
        }
    }

    pub fn push(&mut self, value: usize) {
        self.pointer -= core::mem::size_of::<usize>();
        unsafe { *(self.pointer as *mut usize) = value };
    }

    #[inline(always)]
    pub fn top(&self) -> usize {
        self.top
    }

    #[inline(always)]
    pub fn pointer(&self) -> &usize {
        &self.pointer
    }
}

impl Drop for Stack {
    fn drop(&mut self) {
        memory::free_kernel_stack(KERNEL_STACK_SIZE);
        unsafe { alloc::alloc::dealloc(self.stack.as_mut_ptr(), KERNEL_STACK_LAYOUT) };
    }
}
