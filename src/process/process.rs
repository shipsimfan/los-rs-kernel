use super::Thread;
use crate::{
    logln,
    map::{Map, Mappable, INVALID_ID},
    memory::AddressSpace,
    session::Session,
};

pub struct Process {
    id: usize,
    threads: Map<Thread>,
    address_space: AddressSpace,
    session: Option<*mut Session>,
}

impl Process {
    pub fn new(session: Option<&mut Session>) -> Self {
        Process {
            id: INVALID_ID,
            threads: Map::new(),
            address_space: AddressSpace::new(),
            session: match session {
                Some(session) => Some(session),
                None => None,
            },
        }
    }

    pub fn create_thread(&mut self, entry: usize, context: usize) -> usize {
        let thread = Thread::new(self, entry, context);
        let tid = self.threads.insert(thread);
        super::queue_thread(self.threads.get_mut(tid).unwrap());
        tid
    }

    pub fn remove_thread(&mut self, id: usize) -> bool {
        self.threads.remove(id);
        self.threads.count() == 0
    }

    pub fn set_address_space_as_current(&self) {
        self.address_space.set_as_current()
    }

    pub fn get_session_mut(&mut self) -> Option<&mut Session> {
        match self.session {
            Some(session) => Some(unsafe { &mut *session }),
            None => None,
        }
    }
}

impl Mappable for Process {
    fn id(&self) -> usize {
        self.id
    }

    fn set_id(&mut self, id: usize) {
        self.id = id
    }
}

impl Drop for Process {
    fn drop(&mut self) {
        if self.threads.count() > 0 {
            panic!("Dropping process while it still has threads!");
        }

        self.address_space.free();

        logln!("Dropping process!");
    }
}

unsafe impl Send for Process {}
