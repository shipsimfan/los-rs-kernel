use core::{ffi::c_void, ptr::null_mut};

use super::{exit_thread, Process, ThreadFuncContext, ThreadQueue};
use crate::{
    logln,
    map::{Mappable, INVALID_ID},
};

struct Stack {
    stack: *mut u8,
    top: usize,
    pointer: usize,
}

pub struct Thread {
    id: usize,
    kernel_stack: Stack,
    floating_point_storage: *mut u8,
    process: *mut Process,
    queue: Option<*mut ThreadQueue>,
}

const KERNEL_STACK_SIZE: usize = 32 * 1024;
const FLOATING_POINT_STORAGE_SIZE: usize = 512;

const FLOATING_POINT_STORAGE_LAYOUT: alloc::alloc::Layout =
    unsafe { alloc::alloc::Layout::from_size_align_unchecked(FLOATING_POINT_STORAGE_SIZE, 16) };
const KERNEL_STACK_LAYOUT: alloc::alloc::Layout =
    unsafe { alloc::alloc::Layout::from_size_align_unchecked(KERNEL_STACK_SIZE, 16) };

extern "C" {
    fn float_save(floating_point_storage: *mut u8);
    fn float_load(floating_point_storage: *mut u8);
}

extern "C" fn thread_enter_kernel(entry: *const c_void, context: usize) {
    let entry: ThreadFuncContext = unsafe { core::mem::transmute(entry) };
    entry(context);
    exit_thread();
}

impl Thread {
    pub fn new(process: &mut Process, entry: usize, context: usize) -> Self {
        let mut kernel_stack = Stack::new();
        kernel_stack.push(thread_enter_kernel as usize); // ret address
        kernel_stack.push(0); // push rax
        kernel_stack.push(0); // push rbx
        kernel_stack.push(0); // push rcx
        kernel_stack.push(0); // push rdx
        kernel_stack.push(context); // push rsi
        kernel_stack.push(entry); // push rdi
        kernel_stack.push(0); // push rbp
        kernel_stack.push(0); // push r8
        kernel_stack.push(0); // push r9
        kernel_stack.push(0); // push r10
        kernel_stack.push(0); // push r11
        kernel_stack.push(0); // push r12
        kernel_stack.push(0); // push r13
        kernel_stack.push(0); // push r14
        kernel_stack.push(0); // push r15

        let fps = unsafe { alloc::alloc::alloc_zeroed(FLOATING_POINT_STORAGE_LAYOUT) };

        Thread {
            id: INVALID_ID,
            kernel_stack: kernel_stack,
            floating_point_storage: fps,
            process: process,
            queue: None,
        }
    }

    pub fn save_float(&self) {
        unsafe { float_save(self.floating_point_storage) };
    }

    pub fn load_float(&self) {
        unsafe { float_load(self.floating_point_storage) };
    }

    pub fn set_interrupt_stack(&self) {
        crate::interrupts::set_interrupt_stack(self.kernel_stack.top);
    }

    pub fn set_queue(&mut self, queue: &mut ThreadQueue) {
        self.queue = Some(queue);
    }

    pub fn clear_queue(&mut self) {
        self.queue = None;
    }

    pub fn in_queue(&mut self) -> bool {
        self.queue.is_some()
    }

    pub fn get_stack_pointer_location(&self) -> *mut usize {
        (&self.kernel_stack.pointer) as *const _ as *mut _
    }

    pub fn get_process(&self) -> &'static Process {
        unsafe { &*self.process }
    }

    pub fn get_process_mut(&mut self) -> &'static mut Process {
        unsafe { &mut *self.process }
    }

    pub fn compare_process(&self, other: &Thread) -> bool {
        self.process == other.process
    }
}

impl Mappable for Thread {
    fn id(&self) -> usize {
        self.id
    }

    fn set_id(&mut self, id: usize) {
        self.id = id
    }
}

impl Drop for Thread {
    fn drop(&mut self) {
        match self.queue {
            Some(queue) => unsafe { (*queue).remove(self) },
            None => {}
        }

        unsafe {
            alloc::alloc::dealloc(self.floating_point_storage, FLOATING_POINT_STORAGE_LAYOUT)
        };

        logln!("Dropping Thread");
    }
}

impl Stack {
    pub fn new() -> Self {
        let stack = unsafe { alloc::alloc::alloc_zeroed(KERNEL_STACK_LAYOUT) };

        // Set the kernel stack pointer appropriately
        let top = stack as usize + KERNEL_STACK_SIZE;
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
}

impl Drop for Stack {
    fn drop(&mut self) {
        unsafe { alloc::alloc::dealloc(self.stack, KERNEL_STACK_LAYOUT) };
    }
}
