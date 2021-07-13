use crate::{error, locks::Mutex, map::Map, process::Process};
use alloc::sync::Arc;

pub mod color;
mod console;
mod control;

pub struct Session {
    _id: SID,
    sub: SubSession,
    processes: Map<Process>,
}

pub enum SubSession {
    Console(console::Console),
}

pub type SessionBox = Arc<Mutex<Session>>;
type SID = usize;

static SESSIONS: Mutex<control::SessionControl> = Mutex::new(control::SessionControl::new());

pub fn create_console_session(output_device_path: &str) -> Result<SessionBox, error::Status> {
    let output_device = crate::device::get_device(output_device_path)?;
    Ok(
        (*SESSIONS.lock())
            .create_session(SubSession::Console(console::Console::new(output_device))),
    )
}

impl Session {
    pub fn new(id: SID, sub: SubSession) -> Self {
        Session {
            _id: id,
            sub,
            processes: Map::new(),
        }
    }

    pub fn create_process(&mut self, entry: usize, context: usize) -> usize {
        let new_process = Process::new(Some(self));
        let pid = self.processes.insert(new_process);
        self.processes
            .get_mut(pid)
            .unwrap()
            .create_thread(entry, context);
        pid
    }

    pub fn get_process_mut(&mut self, pid: usize) -> Option<&mut Process> {
        self.processes.get_mut(pid)
    }

    pub fn remove_process(&mut self, id: usize) {
        self.processes.remove(id);
    }

    pub fn get_sub_session_mut(&mut self) -> &mut SubSession {
        &mut self.sub
    }
}
