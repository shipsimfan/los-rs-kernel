use alloc::boxed::Box;

pub struct Stack {
    stack: Box<[u8; KERNEL_STACK_SIZE]>,
    top: usize,
    pointer: usize,
}

const KERNEL_STACK_SIZE: usize = 32 * 1024; // 32 Kilobytes

impl Stack {
    pub fn new() -> Self {
        let stack = Box::new([0; KERNEL_STACK_SIZE]);

        let pointer = stack.as_ptr() as usize;
        let top = pointer + KERNEL_STACK_SIZE;

        Stack {
            stack,
            pointer,
            top,
        }
    }

    #[inline(always)]
    pub fn push(&mut self, value: usize) {
        self.pointer -= core::mem::size_of::<usize>();
        unsafe { *(self.pointer as *mut usize) = value };
    }
}
