pub struct Stack {
    stack: *mut u8,
    top: usize,
    pointer: usize,
}

const KERNEL_STACK_SIZE: usize = 32 * 1024; // 32 Kilobytes
const KERNEL_STACK_LAYOUT: alloc::alloc::Layout =
    unsafe { alloc::alloc::Layout::from_size_align_unchecked(KERNEL_STACK_SIZE, 16) };

impl Stack {
    pub fn new() -> Self {
        let stack = unsafe { alloc::alloc::alloc_zeroed(KERNEL_STACK_LAYOUT) };

        let top = stack as usize + KERNEL_STACK_SIZE;
        let pointer = top;

        Stack {
            stack,
            pointer,
            top,
        }
    }

    pub fn pointer_location(&self) -> &usize {
        &self.pointer
    }

    pub fn top(&self) -> usize {
        self.top
    }

    #[inline(always)]
    pub fn push(&mut self, value: usize) {
        self.pointer -= core::mem::size_of::<usize>();
        unsafe { *(self.pointer as *mut usize) = value };
    }
}

impl Drop for Stack {
    fn drop(&mut self) {
        unsafe { alloc::alloc::dealloc(self.stack, KERNEL_STACK_LAYOUT) };
    }
}
