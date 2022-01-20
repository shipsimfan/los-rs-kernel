use super::{queue::CurrentQueue, stack::Stack, ThreadFuncContext};
use crate::{
    map::{Mappable, INVALID_ID},
    memory::KERNEL_VMA,
    process::{exit_thread, process::ProcessOwner, queue_thread, ProcessReference, ThreadQueue},
};
use core::ffi::c_void;

pub struct ThreadInner {
    id: isize,
    kernel_stack: Stack,
    floating_point_storage: &'static mut [u8],
    process: ProcessOwner,
    queue: Option<CurrentQueue>,
    queue_data: isize,
    exit_queue: ThreadQueue,
    tls_base: usize,
}

const FLOATING_POINT_STORAGE_SIZE: usize = 512;
const FLOATING_POINT_STORAGE_LAYOUT: alloc::alloc::Layout =
    unsafe { alloc::alloc::Layout::from_size_align_unchecked(FLOATING_POINT_STORAGE_SIZE, 16) };

extern "C" {
    fn float_save(floating_point_storage: *mut u8);
    fn float_load(floating_point_storage: *mut u8);
    fn thread_enter_user(entry: *const c_void, context: usize);
    fn set_fs_base(fs_base: usize);
}

extern "C" fn thread_enter_kernel(entry: *const c_void, context: usize) {
    unsafe { crate::critical::leave_local(false) };
    let entry: ThreadFuncContext = unsafe { core::mem::transmute(entry) };
    let status = entry(context);
    exit_thread(status, None);
}

impl ThreadInner {
    pub fn new(process: ProcessOwner, entry: usize, context: usize) -> Self {
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

        ThreadInner {
            id: INVALID_ID,
            kernel_stack: kernel_stack,
            floating_point_storage: fps,
            process,
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
        crate::interrupts::set_interrupt_stack(self.kernel_stack.top());
        unsafe { set_fs_base(self.tls_base) };
    }

    pub unsafe fn set_queue(&mut self, queue: CurrentQueue) {
        self.clear_queue(false);

        self.queue = Some(queue)
    }

    pub unsafe fn clear_queue(&mut self, removed: bool) {
        if !removed {
            let ptr = self as *mut _;
            match &mut self.queue {
                Some(queue) => queue.remove(ptr),
                None => {}
            }
        }

        self.queue = None;
    }

    pub fn set_tls_base(&mut self, new_tls_base: usize) {
        self.tls_base = new_tls_base;
        unsafe { set_fs_base(self.tls_base) };
    }

    pub fn get_stack_pointer_location(&self) -> *const usize {
        self.kernel_stack.pointer() as *const _
    }

    pub fn process(&self) -> ProcessReference {
        self.process.reference()
    }

    pub fn set_queue_data(&mut self, new_data: isize) {
        self.queue_data = new_data;
    }

    pub fn get_queue_data(&self) -> isize {
        self.queue_data
    }

    pub fn get_exit_queue(&mut self) -> CurrentQueue {
        self.exit_queue.into_current_queue()
    }

    pub unsafe fn pre_exit(&mut self, exit_status: isize) {
        while let Some(thread) = self.exit_queue.pop() {
            thread.set_queue_data(exit_status);
            queue_thread(thread);
        }
    }
}

impl Mappable for ThreadInner {
    fn id(&self) -> isize {
        self.id
    }

    fn set_id(&mut self, id: isize) {
        self.id = id
    }
}

impl Drop for ThreadInner {
    fn drop(&mut self) {
        self.process.remove_thread(self.id);

        unsafe {
            self.pre_exit(128);

            self.clear_queue(false);

            alloc::alloc::dealloc(
                self.floating_point_storage.as_mut_ptr(),
                FLOATING_POINT_STORAGE_LAYOUT,
            )
        };
    }
}
