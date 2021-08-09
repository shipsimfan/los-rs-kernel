use crate::{
    error,
    event::Event,
    filesystem::DirectoryDescriptor,
    locks::Mutex,
    map::{Map, Mappable},
    process::{self, Process, StandardIO, StandardIOType},
};
use alloc::{string::ToString, sync::Arc, vec::Vec};

pub mod console;

pub struct Session {
    sub: SubSession,
    processes: Map<Process>,
}

struct SessionHolder {
    sbox: SessionBox,
    id: isize,
}

pub enum SubSession {
    Console(console::Console),
}

pub type SessionBox = Arc<Mutex<Session>>;

static SESSIONS: Mutex<Map<SessionHolder>> = Mutex::new(Map::with_starting_index(1));

pub fn create_console_session(output_device_path: &str) -> error::Result<isize> {
    let output_device = crate::device::get_device(output_device_path)?;
    let new_session = Session::new(SubSession::Console(console::Console::new(output_device)?));
    let sid = SESSIONS.lock().insert(SessionHolder {
        id: 0,
        sbox: Arc::new(Mutex::new(new_session)),
    });

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
        sid,
    )?;

    Ok(sid)
}

pub fn get_session_mut(sid: isize) -> Option<SessionBox> {
    match SESSIONS.lock().get_mut(sid) {
        Some(holder) => Some(holder.sbox.clone()),
        None => None,
    }
}

impl Session {
    pub fn new(sub: SubSession) -> Self {
        Session {
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

impl Mappable for SessionHolder {
    fn id(&self) -> isize {
        self.id
    }

    fn set_id(&mut self, id: isize) {
        self.id = id
    }
}
