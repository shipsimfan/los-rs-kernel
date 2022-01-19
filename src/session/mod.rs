use crate::{
    error,
    event::Event,
    filesystem::DirectoryDescriptor,
    locks::{Spinlock, SpinlockGuard},
    map::{Map, Mappable, INVALID_ID},
    process::{self, ProcessOwner, ProcessReference, StandardIO, StandardIOType, ThreadOwner},
};
use alloc::{string::ToString, sync::Arc, vec::Vec};

pub mod console;

pub struct Session {
    sub: SubSession,
    processes: Map<ProcessReference>,
    id: isize,
}

pub enum SubSession {
    Console(console::Console),
}

#[derive(Clone)]
pub struct SessionBox(Arc<Spinlock<Session>>);

static SESSIONS: Spinlock<Map<SessionBox>> = Spinlock::new(Map::with_starting_index(1));

pub fn create_console_session(output_device_path: &str) -> error::Result<isize> {
    let output_device = crate::device::get_device(output_device_path)?;
    let new_session = Session::new(SubSession::Console(console::Console::new(output_device)?));
    let sid = SESSIONS
        .lock()
        .insert(SessionBox(Arc::new(Spinlock::new(new_session))));

    let mut env = Vec::new();
    env.push("PATH=:1/los/bin".to_string());

    process::execute_session(
        ":1/los/bin/cshell.app",
        Vec::new(),
        env,
        StandardIO::new(
            StandardIOType::Console,
            StandardIOType::Console,
            StandardIOType::Console,
        ),
        Some(sid),
    )?;

    Ok(sid)
}

pub fn get_session_mut(sid: isize) -> Option<SessionBox> {
    SESSIONS.lock().get_mut(sid).map(|sbox| sbox.clone())
}

impl Session {
    pub fn new(sub: SubSession) -> Self {
        Session {
            sub,
            processes: Map::new(),
            id: INVALID_ID,
        }
    }

    pub fn create_process(
        &mut self,
        entry: usize,
        context: usize,
        working_directory: Option<DirectoryDescriptor>,
    ) -> ThreadOwner {
        let new_process = ProcessOwner::new(Some(self.id), working_directory);
        self.processes.insert(new_process.reference());
        new_process.create_thread(entry, context)
    }

    pub fn get_process(&self, pid: isize) -> Option<ProcessReference> {
        self.processes.get(pid).map(|reference| reference.clone())
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

impl SessionBox {
    pub fn lock(&self) -> SpinlockGuard<Session> {
        self.0.lock()
    }
}

impl Mappable for SessionBox {
    fn id(&self) -> isize {
        self.0.lock().id
    }

    fn set_id(&mut self, id: isize) {
        self.0.lock().id = id
    }
}
