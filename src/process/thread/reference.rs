use super::{inner::ThreadInner, CurrentQueue};
use crate::{
    critical::CriticalLock,
    ipc::SignalHandler,
    map::{Mappable, INVALID_ID},
    process::ProcessReference,
};
use alloc::sync::Weak;

#[derive(Clone)]
pub struct ThreadReference(Weak<CriticalLock<ThreadInner>>);

impl ThreadReference {
    pub fn new(thread: Weak<CriticalLock<ThreadInner>>) -> Self {
        ThreadReference(thread)
    }

    pub fn process(&self) -> Option<ProcessReference> {
        match self.0.upgrade() {
            Some(thread) => Some(thread.lock().process()),
            None => None,
        }
    }

    pub fn save_float(&self) {
        match self.0.upgrade() {
            Some(thread) => thread.lock().save_float(),
            None => {}
        }
    }

    pub fn get_queue_data(&self) -> Option<isize> {
        match self.0.upgrade() {
            Some(thread) => Some(thread.lock().get_queue_data()),
            None => None,
        }
    }

    pub fn get_exit_queue(&self) -> Option<CurrentQueue> {
        match self.0.upgrade() {
            Some(thread) => Some(thread.lock().get_exit_queue()),
            None => None,
        }
    }

    pub fn get_stack_pointer_location(&self) -> Option<*const usize> {
        match self.0.upgrade() {
            Some(thread) => Some(thread.lock().get_stack_pointer_location()),
            None => None,
        }
    }

    pub fn set_tls_base(&self, new_tls_base: usize) {
        match self.0.upgrade() {
            Some(thread) => thread.lock().set_tls_base(new_tls_base),
            None => {}
        }
    }

    pub fn raise(&self, signal: u8) {
        match self.0.upgrade() {
            Some(thread) => thread.lock().raise(signal),
            None => {}
        }
    }

    pub fn set_signal_handler(&self, signal: u8, handler: SignalHandler) {
        match self.0.upgrade() {
            Some(thread) => thread.lock().set_signal_handler(signal, handler),
            None => {}
        }
    }

    pub unsafe fn pre_exit(&self, exit_status: isize) {
        match self.0.upgrade() {
            Some(thread) => thread.lock().pre_exit(exit_status),
            None => {}
        }
    }

    pub unsafe fn clear_queue(&self, removed: bool) {
        match self.0.upgrade() {
            Some(thread) => thread.lock().clear_queue(removed),
            None => {}
        }
    }
}

impl Mappable for ThreadReference {
    fn id(&self) -> isize {
        match self.0.upgrade() {
            Some(thread) => thread.lock().id(),
            None => INVALID_ID,
        }
    }

    fn set_id(&mut self, id: isize) {
        match self.0.upgrade() {
            Some(thread) => thread.lock().set_id(id),
            None => {}
        }
    }
}

impl PartialEq for ThreadReference {
    fn eq(&self, other: &Self) -> bool {
        match self.0.upgrade() {
            Some(us) => match other.0.upgrade() {
                Some(other) => us == other,
                None => false,
            },
            None => false,
        }
    }
}
