use crate::LocalState;

use super::enter::{thread_enter_kernel, thread_enter_user};
use alloc::alloc::{alloc_zeroed, dealloc};
use core::alloc::Layout;

pub(in crate::process) struct Stack {
    stack: *mut u8,
    top: usize,
    pointer: usize,
}

const KERNEL_STACK_SIZE: usize = 32 * 1024;
const KERENL_STACK_LAYOUT: Layout = match Layout::from_size_align(KERNEL_STACK_SIZE, 16) {
    Ok(layout) => layout,
    Err(_) => panic!("Invalid kernel stack layout"),
};

impl Stack {
    pub(super) fn new() -> Self {
        let stack = unsafe { alloc_zeroed(KERENL_STACK_LAYOUT) };
        let top = stack as usize + KERNEL_STACK_SIZE;

        Stack {
            stack,
            top,
            pointer: top,
        }
    }

    pub(super) unsafe fn set_interrupt_stack(&self) {
        LocalState::get().gdt().set_interrupt_stack(self.top as u64);
    }

    pub(super) unsafe fn pointer_location(&self) -> *mut usize {
        &self.pointer as *const usize as *mut usize
    }

    pub(super) fn initialize_kernel(&mut self, entry: usize, context: usize) {
        self.push(thread_enter_kernel as usize); // ret address
        self.push_registers(entry, context);
    }

    pub(super) fn initialize_user(&mut self, entry: usize, context: usize) {
        self.push(0x1B); // ss
        self.push(0x7FFFFFFFFFF0); // rsp
        self.push(0x202); // rflags
        self.push(0x23); //cs
        self.push(entry); // rip
        self.push(thread_enter_user as usize); // ret address
        self.push_registers(context, 0);
    }

    fn push_registers(&mut self, rdi: usize, rsi: usize) {
        self.push(0); // push rax
        self.push(0); // push rbx
        self.push(0); // push rcx
        self.push(0); // push rdx
        self.push(rsi); // push rsi
        self.push(rdi); // push rdi
        self.push(0); // push rbp
        self.push(0); // push r8
        self.push(0); // push r9
        self.push(0); // push r10
        self.push(0); // push r11
        self.push(0); // push r12
        self.push(0); // push r13
        self.push(0); // push r14
        self.push(0); // push r15
    }

    fn push(&mut self, value: usize) {
        self.pointer -= core::mem::size_of::<usize>();
        unsafe { *(self.pointer as *mut usize) = value };
    }
}

impl Drop for Stack {
    fn drop(&mut self) {
        unsafe { dealloc(self.stack, KERENL_STACK_LAYOUT) }
    }
}
