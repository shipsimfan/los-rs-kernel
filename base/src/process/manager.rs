use super::{Process, Thread};
use crate::{
    util::{Map, Queue},
    CriticalLock, Mutex,
};
use alloc::sync::{Arc, Weak};

pub struct ProcessManager {
    // TODO: Upgrade to a RWLock
    processes: Mutex<Map<u64, (u64, Weak<Process>)>>,
    waiting_queue: CriticalLock<Queue<Arc<Thread>>>,
}

static PROCESS_MANAGER: ProcessManager = ProcessManager::new();

impl ProcessManager {
    pub fn get<'a>() -> &'a ProcessManager {
        &PROCESS_MANAGER
    }

    pub(self) const fn new() -> Self {
        ProcessManager {
            processes: Mutex::new(Map::new(0)),
            waiting_queue: CriticalLock::new(Queue::new()),
        }
    }
}
