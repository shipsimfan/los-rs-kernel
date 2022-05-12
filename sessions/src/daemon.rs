use crate::Session;
use base::{
    map::{Map, Mappable, INVALID_ID},
    multi_owner::Reference,
};
use process::{Process, ProcessOwner, ProcessTypes};

pub struct DaemonSession<T: ProcessTypes + 'static> {
    processes: Map<Reference<Process<T>>>,
    id: isize,
}

impl<T: ProcessTypes> DaemonSession<T> {
    pub fn new() -> Self {
        DaemonSession {
            processes: Map::new(),
            id: INVALID_ID,
        }
    }
}

impl<T: ProcessTypes> Session<T> for DaemonSession<T> {
    fn push_event(&mut self, _: crate::Event) {}
    fn peek_event(&mut self) -> Option<crate::Event> {
        None
    }
    fn get_event_thread_queue(&self) -> Option<process::CurrentQueue<T>> {
        None
    }
}

impl<T: ProcessTypes> ProcessOwner<T> for DaemonSession<T> {
    fn insert_process(&mut self, process: Reference<Process<T>>) {
        self.processes.insert(process);
    }

    fn remove_process(&mut self, id: isize) {
        self.processes.remove(id);
    }
}

impl<T: ProcessTypes> Mappable for DaemonSession<T> {
    fn set_id(&mut self, id: isize) {
        self.id = id;
    }

    fn id(&self) -> isize {
        self.id
    }
}

unsafe impl<T: ProcessTypes + Send> Send for DaemonSession<T> {}
