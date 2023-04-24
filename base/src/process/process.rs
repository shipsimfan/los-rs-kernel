use super::{thread::ThreadInner, Thread};
use crate::{
    AddressSpace, CriticalLock, Map, Mappable, MappableMut, ProcessManager, StandardError,
    ThreadQueue,
};
use alloc::{
    borrow::Cow,
    sync::{Arc, Weak},
};

pub struct Process {
    id: u64,
    name: Cow<'static, str>,

    address_space: AddressSpace,

    threads: CriticalLock<Map<u64, (u64, Weak<ThreadInner>)>>,

    exit_queue: ThreadQueue,
}

impl Process {
    pub(super) fn new<S: Into<Cow<'static, str>>>(name: S) -> Self {
        Process {
            id: 0,
            name: name.into(),

            address_space: AddressSpace::new(),

            threads: CriticalLock::new(Map::new(0)),

            exit_queue: ThreadQueue::new(),
        }
    }

    pub fn id(&self) -> u64 {
        self.id
    }

    pub fn name(&self) -> &Cow<'static, str> {
        &self.name
    }

    pub fn create_thread(self: &Arc<Self>, entry: usize, context: usize) -> Thread {
        let mut thread_inner = ThreadInner::new(self.clone(), entry, context);
        let mut thread = None;
        self.threads.lock().insert_f(|id| {
            thread_inner.set_id(id);
            let arc = Arc::new(thread_inner);
            thread = Some(Thread::new(arc.clone()));
            (*id, Arc::downgrade(&arc))
        });
        thread.unwrap()
    }

    pub fn get_thread<F, T>(&self, id: u64, f: F) -> Result<T, StandardError>
    where
        F: FnOnce(&Thread) -> T,
    {
        match self.threads.lock().get(id) {
            Some((_, thread)) => thread.upgrade().map(|thread| f(&Thread::new(thread))),
            None => None,
        }
        .ok_or(StandardError::ThreadNotFound)
    }

    pub fn kill(&self, exit_code: isize) {
        for (_, thread) in &*self.threads.lock() {
            thread.upgrade().map(|thread| thread.kill(exit_code));
        }

        self.wake_exit_queue(exit_code);
    }

    pub fn wait(&self) {
        ProcessManager::get().r#yield(Some(self.exit_queue.clone().lock()));
    }

    pub(super) fn address_space(&self) -> &AddressSpace {
        &self.address_space
    }

    pub(super) fn remove_thread(&self, id: u64, exit_code: isize) {
        let mut threads = self.threads.lock();
        threads.remove(&id);
        let count = threads.len();
        drop(threads);

        if count == 0 {
            self.wake_exit_queue(exit_code);
        }
    }

    fn wake_exit_queue(&self, exit_code: isize) {
        let process_manager = ProcessManager::get();
        self.exit_queue.pop_all(|thread| {
            thread.set_queue_data(exit_code);
            process_manager.queue_thread(thread);
        });
    }
}

impl Mappable<u64> for Process {
    fn id(&self) -> &u64 {
        &self.id
    }
}

impl MappableMut<u64> for Process {
    fn set_id(&mut self, id: &u64) {
        self.id = *id;
    }
}

impl Drop for Process {
    fn drop(&mut self) {
        ProcessManager::get().remove_process(self.id)
    }
}
