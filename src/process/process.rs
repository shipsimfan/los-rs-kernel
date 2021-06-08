use super::{Lock, ProcessBox, Thread, WeakThreadBox, TID};
use crate::{logln, session::SessionBox};
use alloc::{sync::Arc, vec::Vec};

pub struct Process {
    id: PID,
    next_id: TID,
    threads: Vec<WeakThreadBox>,
    address_space: crate::memory::AddressSpace,
    session: Option<SessionBox>,
}

pub type PID = usize;

impl Process {
    pub fn new(id: PID, session: Option<&SessionBox>) -> Self {
        Process {
            id: id,
            next_id: 0,
            threads: Vec::new(),
            address_space: crate::memory::AddressSpace::new(),
            session: match session {
                None => None,
                Some(sb) => Some(sb.clone()),
            },
        }
    }

    pub fn create_thread(&mut self, self_box: &ProcessBox, entry: usize, context: usize) {
        // Create the thread
        let new_thread = Arc::new(Lock::new(Thread::new(
            self.next_id,
            self_box,
            entry,
            context,
        )));
        self.threads.push(Arc::downgrade(&new_thread));
        self.next_id += 1;
        super::queue_thread(new_thread);
    }

    pub fn remove_dead_threads(&mut self) {
        self.threads.retain(|x| x.upgrade().is_some());
    }

    pub fn set_address_space_as_current(&self) {
        self.address_space.set_as_current()
    }

    #[allow(dead_code)]
    pub fn get_session(&self) -> Option<SessionBox> {
        self.session.clone()
    }

    #[allow(dead_code)]
    pub fn kill(&mut self) {
        for t in &self.threads {
            match t.upgrade() {
                None => {}
                Some(thread) => thread.lock().kill(),
            }
        }
    }
}

impl Drop for Process {
    fn drop(&mut self) {
        self.remove_dead_threads();

        if self.threads.len() > 0 {
            panic!("Dropping process while it still has threads!");
        }

        self.address_space.free();

        logln!("Dropping process {}", self.id);
    }
}
