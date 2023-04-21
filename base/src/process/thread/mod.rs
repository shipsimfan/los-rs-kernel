use super::Process;
use crate::{memory::KERNEL_VMA, Mappable, MappableMut, ProcessManager, ThreadQueue};
use alloc::{borrow::Cow, sync::Arc};
use core::{
    ops::Deref,
    sync::atomic::{AtomicBool, AtomicIsize, Ordering},
};
use stack::Stack;

mod enter;
mod floats;
mod stack;

pub(super) use floats::FloatingPointStorage;

pub struct Thread(Arc<ThreadInner>);

pub struct ThreadInner {
    id: u64,
    name: Option<Cow<'static, str>>,

    process: Arc<Process>,

    floating_point_storage: FloatingPointStorage,
    stack: Stack,

    queue_data: AtomicIsize,

    killed: AtomicBool,
    exit_queue: ThreadQueue,
}

impl ThreadInner {
    pub(super) fn new(process: Arc<Process>, entry: usize, context: usize) -> Self {
        let mut stack = Stack::new();
        if entry >= KERNEL_VMA {
            stack.initialize_kernel(entry, context);
        } else {
            stack.initialize_user(entry, context);
        }

        ThreadInner {
            id: 0,
            name: None,

            process,

            floating_point_storage: FloatingPointStorage::new(),
            stack,

            queue_data: AtomicIsize::new(0),

            killed: AtomicBool::new(false),
            exit_queue: ThreadQueue::new(),
        }
    }

    pub fn id(&self) -> u64 {
        self.id
    }

    pub fn name(&self) -> Option<&Cow<'static, str>> {
        self.name.as_ref()
    }

    pub fn process(&self) -> &Process {
        &self.process
    }

    pub fn is_killed(&self) -> bool {
        self.killed.load(Ordering::Acquire)
    }

    pub fn kill(&self, exit_code: isize) {
        if self.killed.swap(true, Ordering::Acquire) {
            return;
        }

        let process_manager = ProcessManager::get();

        self.exit_queue.pop_all(|thread| {
            thread.queue_data.store(exit_code, Ordering::Release);
            process_manager.queue_thread(thread);
        })
    }

    pub(in crate::process) fn process_arc(&self) -> &Arc<Process> {
        &self.process
    }

    pub(in crate::process) unsafe fn save_float(&self) {
        self.floating_point_storage.save_float()
    }

    pub(in crate::process) unsafe fn load_float(&self) {
        self.floating_point_storage.load_float()
    }

    pub(in crate::process) unsafe fn set_interrupt_stack(&self) {
        self.stack.set_interrupt_stack()
    }

    pub(in crate::process) unsafe fn stack_pointer_location(&self) -> *mut usize {
        self.stack.pointer_location()
    }
}

impl Mappable<u64> for ThreadInner {
    fn id(&self) -> &u64 {
        &self.id
    }
}

impl MappableMut<u64> for ThreadInner {
    fn set_id(&mut self, id: &u64) {
        self.id = *id;
    }
}

impl Thread {
    pub(super) fn new(inner: Arc<ThreadInner>) -> Self {
        Thread(inner)
    }
}

impl Deref for Thread {
    type Target = ThreadInner;

    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}

impl Mappable<u64> for Thread {
    fn id(&self) -> &u64 {
        &self.0.id
    }
}

impl Drop for ThreadInner {
    fn drop(&mut self) {
        self.process.remove_thread(self.id)
    }
}
