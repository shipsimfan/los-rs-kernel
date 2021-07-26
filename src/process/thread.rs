use core::ffi::c_void;

use super::{exit_thread, Process, ThreadFuncContext, ThreadQueue};
use crate::{
    map::{Mappable, INVALID_ID},
    memory::KERNEL_VMA,
};
use core::sync::atomic::{AtomicPtr, Ordering};

struct Stack {
    stack: &'static mut [u8],
    top: usize,
    pointer: usize,
}

pub struct Thread {
    id: isize,
    kernel_stack: Stack,
    floating_point_storage: &'static mut [u8],
    process: AtomicPtr<Process>,
    queue: Option<AtomicPtr<ThreadQueue>>,
    queue_data: isize,
    exit_queue: ThreadQueue,
    tls_base: usize,
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
    fn thread_enter_user(entry: *const c_void, context: usize);
    fn set_fs_base(fs_base: usize);
}

extern "C" fn thread_enter_kernel(entry: *const c_void, context: usize) {
    let entry: ThreadFuncContext = unsafe { core::mem::transmute(entry) };
    let status = entry(context);
    exit_thread(status);
}

impl Thread {
    pub fn new(process: &mut Process, entry: usize, context: usize) -> Self {
        let mut kernel_stack = Stack::new();
        if entry >= KERNEL_VMA {
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
        } else {
            kernel_stack.push(0x1B); // ss
            kernel_stack.push(0x7FFFFFFFFFF0); // rsp
            kernel_stack.push(0x202); // rflags
            kernel_stack.push(0x23); //cs
            kernel_stack.push(entry); // rip
            kernel_stack.push(thread_enter_user as usize); // ret address
            kernel_stack.push(0); // push rax
            kernel_stack.push(0); // push rbx
            kernel_stack.push(0); // push rcx
            kernel_stack.push(0); // push rdx
            kernel_stack.push(0); // push rsi
            kernel_stack.push(context); // push rdi
            kernel_stack.push(0); // push rbp
            kernel_stack.push(0); // push r8
            kernel_stack.push(0); // push r9
            kernel_stack.push(0); // push r10
            kernel_stack.push(0); // push r11
            kernel_stack.push(0); // push r12
            kernel_stack.push(0); // push r13
            kernel_stack.push(0); // push r14
            kernel_stack.push(0); // push r15
        }

        let fps = unsafe { alloc::alloc::alloc_zeroed(FLOATING_POINT_STORAGE_LAYOUT) };
        let fps =
            unsafe { core::slice::from_raw_parts_mut(fps, FLOATING_POINT_STORAGE_LAYOUT.size()) };

        Thread {
            id: INVALID_ID,
            kernel_stack: kernel_stack,
            floating_point_storage: fps,
            process: AtomicPtr::new(process),
            queue: None,
            queue_data: 0,
            exit_queue: ThreadQueue::new(),
            tls_base: 0,
        }
    }

    pub fn save_float(&mut self) {
        unsafe { float_save(self.floating_point_storage.as_mut_ptr()) };
    }

    pub fn load_float(&mut self) {
        unsafe { float_load(self.floating_point_storage.as_mut_ptr()) };
    }

    pub fn set_interrupt_stack(&self) {
        crate::interrupts::set_interrupt_stack(self.kernel_stack.top);
        unsafe { set_fs_base(self.tls_base) };
    }

    pub fn set_queue(&mut self, queue: &mut ThreadQueue) {
        self.queue = Some(AtomicPtr::new(queue));
    }

    pub fn set_tls_base(&mut self, new_tls_base: usize) {
        self.tls_base = new_tls_base;
        unsafe { set_fs_base(self.tls_base) };
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

    pub fn get_kernel_stack_top(&self) -> usize {
        self.kernel_stack.top
    }

    pub fn get_process(&self) -> &'static Process {
        unsafe { &*self.process.load(Ordering::Acquire) }
    }

    pub fn get_process_mut(&mut self) -> &'static mut Process {
        unsafe { &mut *self.process.load(Ordering::Acquire) }
    }

    pub fn compare_process(&self, other: &Thread) -> bool {
        self.process.load(Ordering::Acquire) == other.process.load(Ordering::Acquire)
    }

    pub fn set_queue_data(&mut self, new_data: isize) {
        self.queue_data = new_data;
    }

    pub fn get_queue_data(&self) -> isize {
        self.queue_data
    }

    pub fn insert_into_exit_queue(&mut self, thread: &mut Thread) {
        self.exit_queue.push(thread);
    }

    pub unsafe fn pre_exit(&mut self, exit_status: isize) {
        while let Some(thread) = self.exit_queue.pop_mut() {
            thread.set_queue_data(exit_status);
            super::queue_thread_cli(thread);
        }
    }
}

impl Mappable for Thread {
    fn id(&self) -> isize {
        self.id
    }

    fn set_id(&mut self, id: isize) {
        self.id = id
    }
}

impl Drop for Thread {
    fn drop(&mut self) {
        unsafe { self.pre_exit(128) };

        match &self.queue {
            Some(queue) => unsafe { (*queue.load(Ordering::Acquire)).remove(self) },
            None => {}
        }

        unsafe {
            alloc::alloc::dealloc(
                self.floating_point_storage.as_mut_ptr(),
                FLOATING_POINT_STORAGE_LAYOUT,
            )
        };
    }
}

impl Stack {
    pub fn new() -> Self {
        let stack = unsafe { alloc::alloc::alloc_zeroed(KERNEL_STACK_LAYOUT) };
        let stack = unsafe { core::slice::from_raw_parts_mut(stack, KERNEL_STACK_LAYOUT.size()) };

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
}

impl Drop for Stack {
    fn drop(&mut self) {
        unsafe { alloc::alloc::dealloc(self.stack.as_mut_ptr(), KERNEL_STACK_LAYOUT) };
    }
}
