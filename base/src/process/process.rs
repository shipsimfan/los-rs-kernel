use super::{thread::ThreadInner, Thread};
use crate::{AddressSpace, CriticalLock, Map, Mappable, MappableMut, ProcessManager};
use alloc::{
    borrow::Cow,
    sync::{Arc, Weak},
};

pub struct Process {
    id: u64,
    name: Cow<'static, str>,

    address_space: AddressSpace,

    threads: CriticalLock<Map<u64, (u64, Weak<ThreadInner>)>>,
}

impl Process {
    pub(super) fn new<S: Into<Cow<'static, str>>>(name: S) -> Self {
        Process {
            id: 0,
            name: name.into(),

            address_space: AddressSpace::new(),

            threads: CriticalLock::new(Map::new(0)),
        }
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

    pub(super) fn address_space(&self) -> &AddressSpace {
        &self.address_space
    }

    pub(super) fn remove_thread(&self, id: u64) {
        self.threads.lock().remove(&id);
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
