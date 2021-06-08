use crate::{locks::Mutex, process};
use alloc::{sync::Arc, vec::Vec};

pub mod color;
mod console;
mod control;
mod desktop;

pub struct Session {
    _id: SID,
    _sub: SessionType,
    next_id: process::PID,
    processes: Vec<process::WeakProcessBox>,
}

pub enum SessionType {
    Console(console::Console),
    Desktop(desktop::Desktop),
}

pub type SessionBox = Arc<Mutex<Session>>;
type SID = usize;

static SESSIONS: Mutex<control::SessionControl> = Mutex::new(control::SessionControl::new());

pub fn create_console_session() -> SessionBox {
    (*SESSIONS.lock()).create_session(SessionType::Console(console::Console::new()))
}

#[allow(dead_code)]
pub fn create_desktop_session() -> SessionBox {
    (*SESSIONS.lock()).create_session(SessionType::Desktop(desktop::Desktop::new()))
}

impl Session {
    pub fn new(id: SID, sub: SessionType) -> Self {
        Session {
            _id: id,
            _sub: sub,
            next_id: 0,
            processes: Vec::new(),
        }
    }

    pub fn create_process(&mut self, entry: usize, context: usize, session: Option<&SessionBox>) {
        let new_process = Arc::new(Mutex::new(process::Process::new(self.next_id, session)));
        new_process
            .lock()
            .create_thread(&new_process, entry, context);
        self.processes.push(Arc::downgrade(&new_process));
        self.next_id += 1;
    }
}
