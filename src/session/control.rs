use super::{Session, SessionBox, SubSession, SID};
use crate::locks::Mutex;
use alloc::{sync::Arc, vec::Vec};

pub struct SessionControl {
    sessions: Vec<SessionBox>,
    next_id: SID,
}

impl SessionControl {
    pub const fn new() -> Self {
        SessionControl {
            sessions: Vec::new(),
            next_id: 1,
        }
    }

    pub fn create_session(&mut self, sub: SubSession) -> SessionBox {
        let new_session = Arc::new(Mutex::new(Session::new(self.next_id, sub)));
        self.next_id += 1;
        self.sessions.push(new_session.clone());
        new_session
    }
}
