use super::{exit_thread, ProcessBox, ThreadFuncContext};
use crate::logln;
use alloc::vec::Vec;
use core::mem::size_of;

#[repr(C, align(16))]
struct usizeA16(usize);

struct Stack {
    _stack: Vec<usizeA16>,
    top: usize,
    pointer: usize,
}

pub struct Thread {
    id: TID,
    kernel_stack: Stack,
    floating_point_storage: Vec<usizeA16>,
    process: ProcessBox,
    kill_flag: bool,
}

pub type TID = usize;

pub const KERNEL_STACK_SIZE: usize = 32 * 1024;
pub const FLOATING_POINT_STORAGE_SIZE: usize = 512;
pub const KERNEL_STACK_SIZE_USIZE: usize = KERNEL_STACK_SIZE / size_of::<usizeA16>();
pub const FLOATING_POINT_STORAGE_SIZE_USIZE: usize =
    FLOATING_POINT_STORAGE_SIZE / size_of::<usizeA16>();

extern "C" {
    fn float_save(floating_point_storage: *const usizeA16);
    fn float_load(floating_point_storage: *const usizeA16);
}

#[allow(improper_ctypes_definitions)]
extern "C" fn thread_enter_kernel(entry: ThreadFuncContext, context: usize) {
    entry(context);
    exit_thread();
}

impl Thread {
    pub fn new(id: TID, process: &ProcessBox, entry: usize, context: usize) -> Self {
        let mut thread = Thread {
            id: id,
            kernel_stack: Stack::new(),
            floating_point_storage: Vec::with_capacity(FLOATING_POINT_STORAGE_SIZE_USIZE),
            process: process.clone(),
            kill_flag: false,
        };

        // Prepare the stack
        thread.kernel_stack.push(thread_enter_kernel as usize); // ret address
        thread.kernel_stack.push(0); // push rax
        thread.kernel_stack.push(0); // push rbx
        thread.kernel_stack.push(0); // push rcx
        thread.kernel_stack.push(0); // push rdx
        thread.kernel_stack.push(context); // push rsi
        thread.kernel_stack.push(entry); // push rdi
        thread.kernel_stack.push(0); // push rbp
        thread.kernel_stack.push(0); // push r8
        thread.kernel_stack.push(0); // push r9
        thread.kernel_stack.push(0); // push r10
        thread.kernel_stack.push(0); // push r11
        thread.kernel_stack.push(0); // push r12
        thread.kernel_stack.push(0); // push r13
        thread.kernel_stack.push(0); // push r14
        thread.kernel_stack.push(0); // push r15

        thread
    }

    pub fn save_float(&self) {
        unsafe { float_save(self.floating_point_storage.as_ptr()) };
    }

    pub fn load_float(&self) {
        unsafe { float_load(self.floating_point_storage.as_ptr()) };
    }

    pub fn set_interrupt_stack(&self) {
        crate::interrupts::set_interrupt_stack(self.kernel_stack.top);
    }

    pub fn get_stack_pointer_location(&self) -> *mut usize {
        (&self.kernel_stack.pointer) as *const _ as *mut _
    }

    pub fn get_process(&self) -> ProcessBox {
        self.process.clone()
    }

    pub fn kill(&mut self) {
        self.kill_flag = true;
    }

    pub fn is_killed(&self) -> bool {
        self.kill_flag
    }
}

impl Drop for Thread {
    fn drop(&mut self) {
        logln!("Dropping thread {}", self.id);
    }
}

impl Stack {
    pub fn new() -> Self {
        let mut stack = Vec::<usizeA16>::with_capacity(KERNEL_STACK_SIZE_USIZE);

        // Zero and page the kernel stack
        let mut i = 0;
        while i < KERNEL_STACK_SIZE {
            stack.push(usizeA16(0));
            i += 1;
        }

        // Set the kernel stack pointer appropriately
        let top = stack.as_ptr() as usize + KERNEL_STACK_SIZE;
        Stack {
            _stack: stack,
            top: top,
            pointer: top,
        }
    }

    pub fn push(&mut self, value: usize) {
        self.pointer -= core::mem::size_of::<usize>();
        unsafe { *(self.pointer as *mut usize) = value };
    }

    pub fn _pop(&mut self) -> usize {
        let ret = unsafe { *(self.pointer as *const usize) };
        self.pointer += core::mem::size_of::<usize>();
        ret
    }
}
