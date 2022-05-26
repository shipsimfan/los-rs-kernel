use crate::{
    execution::post_yield, process::Process, queue_thread, thread_queue::ThreadQueue, ProcessTypes,
};
use alloc::vec::Vec;
use base::{
    map::{Mappable, INVALID_ID},
    multi_owner::{Owner, Reference},
};
use core::ffi::c_void;
use stack::Stack;

mod current_queue;
mod floating_point_storage;
mod stack;

pub type ThreadFunction = fn(context: usize) -> isize;

pub use current_queue::{CurrentQueue, QueueAccess};

use self::floating_point_storage::FloatingPointStorage;

#[derive(PartialEq, Eq)]
enum InterruptState {
    NotInterruptable,
    Interruptable,
    Interrupted,
}

pub struct Thread<T: ProcessTypes + 'static> {
    id: isize,
    kernel_stack: Stack,
    floating_point_storage: FloatingPointStorage,
    process: Owner<Process<T>>,
    queue: Option<CurrentQueue<T>>,
    queue_data: isize,
    exit_queue: ThreadQueue<T>,
    exit_status: isize,
    interrupt_state: InterruptState,
    self_reference: Option<Reference<Thread<T>>>,
    held_locks: Vec<(*const c_void, unsafe fn(*const c_void))>,
    tls_base: usize,
}

extern "C" {
    fn float_save(floating_point_storage: *mut u8);
    fn float_load(floating_point_storage: *mut u8);
}

impl<T: ProcessTypes + 'static> Thread<T> {
    pub fn new(process: Owner<Process<T>>, entry: usize, context: usize) -> Owner<Self> {
        let entry = entry as usize;

        let mut kernel_stack = Stack::new();
        Self::prepare_entry_stack(&mut kernel_stack, entry, context);

        let floating_point_storage = FloatingPointStorage::new();

        let ret = Owner::new(Thread {
            id: INVALID_ID,
            kernel_stack,
            floating_point_storage,
            process,
            queue: None,
            queue_data: 0,
            exit_queue: ThreadQueue::new(),
            exit_status: 128, // Random kill
            interrupt_state: InterruptState::NotInterruptable,
            self_reference: None,
            held_locks: Vec::new(),
            tls_base: 0,
        });

        let reference = ret.as_ref();
        ret.lock(|thread: &mut Thread<T>| thread.self_reference = Some(reference));

        ret
    }

    pub fn process(&self) -> &Owner<Process<T>> {
        &self.process
    }

    pub fn stack_pointer_location(&self) -> &usize {
        self.kernel_stack.pointer_location()
    }

    pub fn stack_top(&self) -> usize {
        self.kernel_stack.top()
    }

    pub fn exit_queue(&self) -> CurrentQueue<T> {
        self.exit_queue.current_queue()
    }

    pub fn queue_data(&self) -> isize {
        self.queue_data
    }

    pub fn tls_base(&self) -> usize {
        self.tls_base
    }

    pub fn signal_interrupted(&mut self) -> bool {
        let ret = self.interrupt_state == InterruptState::Interrupted;
        self.interrupt_state = InterruptState::NotInterruptable;
        ret
    }

    pub fn set_signal_interruptable(&mut self) {
        if self.interrupt_state == InterruptState::NotInterruptable {
            self.interrupt_state = InterruptState::Interruptable;
        }
    }

    pub fn signal_interrupt(&mut self) -> Option<Owner<Thread<T>>> {
        unsafe {
            base::critical::enter_local();

            let thread = if self.interrupt_state == InterruptState::Interruptable {
                let thread = self.clear_queue(false);
                self.interrupt_state = InterruptState::Interrupted;
                thread
            } else {
                None
            };

            base::critical::leave_local();

            thread
        }
    }

    pub fn set_tls_base(&mut self, tls_base: usize) {
        self.tls_base = tls_base;
    }

    pub fn add_lock(&mut self, lock: *const c_void, unlock_func: unsafe fn(*const c_void)) {
        self.held_locks.push((lock, unlock_func));
    }

    pub fn remove_lock(&mut self, lock: *const c_void) {
        self.held_locks.retain(|(l, _)| *l != lock);
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

    pub fn set_current_queue(&mut self, queue: CurrentQueue<T>) {
        self.queue = Some(queue);
    }

    pub unsafe fn clear_queue(&mut self, removed: bool) -> Option<Owner<Thread<T>>> {
        let ret = if !removed {
            match &mut self.queue {
                Some(queue) => queue.remove(self.self_reference.as_ref().unwrap()),
                None => None,
            }
        } else {
            None
        };

        self.queue = None;

        ret
    }

    fn prepare_entry_stack(stack: &mut Stack, entry: usize, context: usize) {
        stack.push(post_yield::<T> as usize); // yield ret address
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
}

impl<T: ProcessTypes> Mappable for Thread<T> {
    fn id(&self) -> isize {
        self.id
    }

    fn set_id(&mut self, id: isize) {
        self.id = id
    }
}

impl<T: ProcessTypes + 'static> Drop for Thread<T> {
    fn drop(&mut self) {
        self.process.lock(|process| process.remove_thread(self.id));

        while let Some(thread) = self.exit_queue.pop() {
            thread.lock(|t| t.set_queue_data(self.exit_status));
            queue_thread(thread);
        }

        unsafe {
            self.clear_queue(false);

            for (lock, unlock_fn) in &self.held_locks {
                unlock_fn(*lock)
            }
        };
    }
}
