use crate::{
    critical::{CriticalLock, CriticalLockGuard},
    error,
    event::{Event, Keycode},
    filesystem::DirectoryDescriptor,
    ipc::{SignalType, Signals},
    map::{Map, Mappable, INVALID_ID},
    process::{
        self, CurrentQueue, ProcessOwner, ProcessReference, StandardIO, StandardIOType, ThreadOwner,
    },
};
use alloc::{
    string::{String, ToString},
    sync::Arc,
    vec::Vec,
};

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
pub struct SessionBox(Arc<CriticalLock<Session>>);

static SESSIONS: CriticalLock<Map<SessionBox>> = CriticalLock::new(Map::with_starting_index(1));

pub fn create_console_session(output_device_path: &str) -> error::Result<isize> {
    let output_device = crate::device::get_device(output_device_path)?;
    let new_session = Session::new(SubSession::Console(console::Console::new(output_device)?));
    let sid = SESSIONS
        .lock()
        .insert(SessionBox(Arc::new(CriticalLock::new(new_session))));

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
        false,
    )?;

    Ok(sid)
}

pub fn get_session(sid: isize) -> Option<SessionBox> {
    SESSIONS.lock().get(sid).map(|sbox| sbox.clone())
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
        name: String,
        signals: Signals,
    ) -> ThreadOwner {
        let new_process = ProcessOwner::new(Some(self.id), working_directory, name, signals);
        self.processes.insert(new_process.reference());
        new_process.create_thread(entry, context)
    }

    pub fn get_process(&self, pid: isize) -> Option<ProcessReference> {
        self.processes.get(pid).map(|reference| reference.clone())
    }

    pub fn get_processes(&self) -> Vec<isize> {
        self.processes.ids()
    }

    pub fn remove_process(&mut self, id: isize) {
        self.processes.remove(id);
    }

    pub fn get_sub_session_mut(&mut self) -> &mut SubSession {
        &mut self.sub
    }

    pub fn push_event(&mut self, event: Event) {
        match self.sub {
            SubSession::Console(_) => match event {
                Event::KeyPress(keycode, keystate) => {
                    match keystate.left_ctrl || keystate.right_ctrl {
                        true => match keycode {
                            Keycode::C => self
                                .processes
                                .for_each(|process| process.raise(SignalType::Interrupt as u8)),
                            _ => {}
                        },
                        false => {}
                    }
                }
                _ => {}
            },
        }

        self.sub.push_event(event);
    }

    pub fn peek_event(&mut self) -> Option<Event> {
        self.sub.peek_event()
    }

    pub fn get_event_thread_queue(&self) -> CurrentQueue {
        self.sub.get_event_thread_queue()
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

    pub fn get_event_thread_queue(&self) -> CurrentQueue {
        match self {
            SubSession::Console(console) => console.get_event_thread_queue(),
        }
    }
}

impl SessionBox {
    pub fn lock(&self) -> CriticalLockGuard<Session> {
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
