use super::Process;
use crate::{
    memory::KERNEL_VMA,
    util::{Mappable, MappableMut},
};
use alloc::{borrow::Cow, sync::Arc};
use core::{ffi::c_void, ops::Deref, sync::atomic::AtomicBool};
use floats::FloatingPointStorage;
use stack::Stack;

mod floats;
mod stack;

pub struct Thread(Arc<ThreadInner>);

pub struct ThreadInner {
    id: u64,
    name: Cow<'static, str>,

    parent: Arc<Process>,

    floating_point_storage: FloatingPointStorage,
    stack: Stack,

    killed: AtomicBool,
}

extern "C" {
    fn thread_enter_kernel(entry: *const c_void, context: usize);
    fn thread_enter_user(entry: *const c_void, context: usize);
}

impl ThreadInner {
    pub(super) fn new<S: Into<Cow<'static, str>>>(
        name: S,
        process: Arc<Process>,
        entry: usize,
        context: usize,
    ) -> Self {
        let mut stack = Stack::new();
        if entry >= KERNEL_VMA {
            stack.initialize_kernel(entry, context);
        } else {
            stack.initialize_user(entry, context);
        }

        ThreadInner {
            id: 0,
            name: name.into(),

            parent: process,

            floating_point_storage: FloatingPointStorage::new(),
            stack,

            killed: AtomicBool::new(false),
        }
    }

    pub fn is_killed(&self) -> bool {
        self.killed.load(core::sync::atomic::Ordering::Acquire)
    }

    pub fn name(&self) -> &Cow<'static, str> {
        &self.name
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
    pub(super) fn new<S: Into<Cow<'static, str>>>(inner: ThreadInner) -> Self {
        Thread(Arc::new(inner))
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
