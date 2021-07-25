use crate::{
    error, event::Event, filesystem::DirectoryDescriptor, locks::Mutex, map::Map, process::Process,
};
use alloc::sync::Arc;

pub mod color;
mod console;
mod control;

pub use console::CONSOLE_IOCTRL_CLEAR;

pub struct Session {
    _id: isize,
    sub: SubSession,
    processes: Map<Process>,
}

pub enum SubSession {
    Console(console::Console),
}

pub type SessionBox = Arc<Mutex<Session>>;

static SESSIONS: Mutex<control::SessionControl> = Mutex::new(control::SessionControl::new());

pub fn create_console_session(output_device_path: &str) -> error::Result<SessionBox> {
    let output_device = crate::device::get_device(output_device_path)?;
    Ok(
        (*SESSIONS.lock())
            .create_session(SubSession::Console(console::Console::new(output_device))),
    )
}

impl Session {
    pub fn new(id: isize, sub: SubSession) -> Self {
        Session {
            _id: id,
            sub,
            processes: Map::new(),
        }
    }

    pub fn create_process(
        &mut self,
        entry: usize,
        context: usize,
        working_directory: Option<DirectoryDescriptor>,
        self_box: SessionBox,
    ) -> isize {
        let new_process = Process::new(Some(self_box), working_directory);
        let pid = self.processes.insert(new_process);
        self.processes
            .get_mut(pid)
            .unwrap()
            .create_thread(entry, context);
        pid
    }

    pub fn get_process_mut(&mut self, pid: isize) -> Option<&mut Process> {
        self.processes.get_mut(pid)
    }

    pub fn remove_process(&mut self, id: isize) {
        self.processes.remove(id);
    }

    pub fn get_sub_session_mut(&mut self) -> &mut SubSession {
        &mut self.sub
    }

    pub fn push_event(&mut self, event: Event) {
        self.sub.push_event(event);
    }

    pub fn peek_event(&mut self) -> Option<Event> {
        self.sub.peek_event()
    }
}

impl SubSession {
    pub fn push_event(&mut self, event: Event) {
        match self {
            SubSession::Console(console) => console.push_event(event),
        }
    }

    pub fn peek_event(&mut self) -> Option<Event> {
        match self {
            SubSession::Console(console) => console.peek_event(),
        }
    }
}
