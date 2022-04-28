use crate::{
    process::{Process, Signals},
    queue_thread,
    thread_queue::ThreadQueue,
    ProcessOwner,
};
use base::{
    map::{Mappable, INVALID_ID},
    multi_owner::{Owner, Reference},
};
use core::ffi::c_void;
use memory::KERNEL_VMA;
use stack::Stack;

mod current_queue;
mod floating_point_storage;
mod stack;

pub type ThreadFunction = fn(context: usize) -> isize;

pub use current_queue::CurrentQueue;

use self::floating_point_storage::FloatingPointStorage;

#[allow(unused)]
enum InterruptState {
    NotInterruptable,
    Interruptable,
    Interrupted,
}

pub struct Thread<O: ProcessOwner<D, S> + 'static, D: 'static, S: Signals + 'static> {
    id: isize,
    kernel_stack: Stack,
    floating_point_storage: FloatingPointStorage,
    process: Owner<Process<O, D, S>>,
    queue: Option<CurrentQueue<O, D, S>>,
    queue_data: isize,
    exit_queue: ThreadQueue<O, D, S>,
    exit_status: isize,
    _interrupt_state: InterruptState,
}

extern "C" {
    fn thread_enter_user(context: usize);
    fn thread_enter_kernel(entry: *const c_void, context: usize);
    fn float_save(floating_point_storage: *mut u8);
    fn float_load(floating_point_storage: *mut u8);
}

impl<O: ProcessOwner<D, S>, D, S: Signals> Thread<O, D, S> {
    pub fn new(
        process: Owner<Process<O, D, S>>,
        entry: ThreadFunction,
        context: usize,
    ) -> Owner<Self> {
        let entry = entry as usize;

        let mut kernel_stack = Stack::new();
        if entry >= KERNEL_VMA {
            Self::prepare_kernel_entry_stack(&mut kernel_stack, entry, context);
        } else {
            Self::prepare_user_entry_stack(&mut kernel_stack, entry, context);
        }

        let floating_point_storage = FloatingPointStorage::new();

        Owner::new(Thread {
            id: INVALID_ID,
            kernel_stack,
            floating_point_storage,
            process,
            queue: None,
            queue_data: 0,
            exit_queue: ThreadQueue::new(),
            exit_status: 128, // Random kill
            _interrupt_state: InterruptState::NotInterruptable,
        })
    }

    pub fn process(&self) -> Reference<Process<O, D, S>> {
        self.process.as_ref()
    }

    pub fn stack_pointer_location(&self) -> &usize {
        self.kernel_stack.pointer_location()
    }

    pub fn stack_top(&self) -> usize {
        self.kernel_stack.top()
    }

    pub fn set_queue_data(&mut self, queue_data: isize) {
        self.queue_data = queue_data;
    }

    pub fn set_exit_status(&mut self, exit_status: isize) {
        self.exit_status = exit_status;
    }

    pub fn save_float(&mut self) {
        unsafe { float_save(self.floating_point_storage.as_mut_ptr()) }
    }

    pub fn load_float(&mut self) {
        unsafe { float_load(self.floating_point_storage.as_mut_ptr()) }
    }

    pub unsafe fn clear_queue(&mut self, removed: bool) -> Option<Owner<Thread<O, D, S>>> {
        let ret = if !removed {
            let ptr = self as *mut _;
            match &mut self.queue {
                Some(queue) => queue.remove(ptr),
                None => None,
            }
        } else {
            None
        };

        self.queue = None;

        ret
    }

    fn prepare_kernel_entry_stack(stack: &mut Stack, entry: usize, context: usize) {
        stack.push(thread_enter_kernel as usize); // ret address
        stack.push(0); // push rax
        stack.push(0); // push rbx
        stack.push(0); // push rcx
        stack.push(0); // push rdx
        stack.push(context); // push rsi
        stack.push(entry); // push rdi
        stack.push(0); // push rbp
        stack.push(0); // push r8
        stack.push(0); // push r9
        stack.push(0); // push r10
        stack.push(0); // push r11
        stack.push(0); // push r12
        stack.push(0); // push r13
        stack.push(0); // push r14
        stack.push(0); // push r15
    }

    fn prepare_user_entry_stack(stack: &mut Stack, entry: usize, context: usize) {
        stack.push(thread_enter_user as usize); // ret address
        stack.push(0); // push rax
        stack.push(0); // push rbx
        stack.push(entry); // push rcx (RIP)
        stack.push(0); // push rdx
        stack.push(0); // push rsi
        stack.push(context); // push rdi
        stack.push(0); // push rbp
        stack.push(0); // push r8
        stack.push(0); // push r9
        stack.push(0); // push r10
        stack.push(0x202); // push r11 (RFLAGS)
        stack.push(0); // push r12
        stack.push(0); // push r13
        stack.push(0); // push r14
        stack.push(0); // push r15
    }
}

impl<O: ProcessOwner<D, S>, D, S: Signals> Mappable for Thread<O, D, S> {
    fn id(&self) -> isize {
        self.id
    }

    fn set_id(&mut self, id: isize) {
        self.id = id
    }
}

impl<O: ProcessOwner<D, S> + 'static, D: 'static, S: Signals + 'static> Drop for Thread<O, D, S> {
    fn drop(&mut self) {
        self.process.lock(|process| process.remove_thread(self.id));

        while let Some(thread) = self.exit_queue.pop() {
            thread.lock(|t| t.set_queue_data(self.exit_status));
            queue_thread(thread);
        }

        unsafe { self.clear_queue(false) };
    }
}
