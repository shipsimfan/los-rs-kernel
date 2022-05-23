#![no_std]

use alloc::boxed::Box;
use base::{
    log_info,
    map::{Map, Mappable},
    multi_owner::{Owner, Reference},
};
use device::Device;
use process::{CurrentQueue, Mutex, ProcessOwner, ProcessTypes};

extern crate alloc;

mod console;
mod daemon;
mod event;

pub use console::*;
pub use daemon::DaemonSession;
pub use event::Event;

pub trait Session<T: ProcessTypes>: ProcessOwner<T> + Mappable + Send {
    fn push_event(&mut self, event: Event);
    fn peek_event(&mut self) -> Option<Event>;
    fn get_event_thread_queue(&self) -> Option<CurrentQueue<T>>;

    fn as_console(&mut self) -> Option<&mut ConsoleSession<T>> {
        None
    }
}

pub const DAEMON_SESSION_ID: usize = 0;
pub const CONSOLE_SESSION_ID: usize = 1;

const MODULE_NAME: &str = "Sessions";

static mut SESSIONS_INITIALIZED: bool = false;

process::static_generic!(
    process::Mutex<
        base::map::Map<base::multi_owner::Owner<alloc::boxed::Box<dyn crate::Session<T>>>>,
        T,
    >,
    sessions
);

pub fn initialize<T: ProcessTypes + 'static>() {
    log_info!("Initializing . . .");

    unsafe {
        assert!(!SESSIONS_INITIALIZED);
        SESSIONS_INITIALIZED = true;
    }

    sessions::initialize::<T>(Mutex::new(Map::with_starting_index(1)));

    log_info!("Initialized!");
}

pub fn create_console_session<T: ProcessTypes>(
    output_device: Reference<Box<dyn Device>, Mutex<Box<dyn Device>, T>>,
) -> base::error::Result<Owner<Box<dyn Session<T>>>> {
    let new_session = ConsoleSession::new(output_device)?;

    sessions::get().lock().insert(new_session.clone());

    Ok(new_session)
}

impl<T: ProcessTypes> ProcessOwner<T> for Box<dyn Session<T>> {
    fn insert_process(&mut self, process: base::multi_owner::Reference<process::Process<T>>) {
        self.as_mut().insert_process(process)
    }

    fn remove_process(&mut self, id: isize) {
        self.as_mut().remove_process(id)
    }
}

impl<T: ProcessTypes> Mappable for Box<dyn Session<T>> {
    fn set_id(&mut self, id: isize) {
        self.as_mut().set_id(id)
    }

    fn id(&self) -> isize {
        self.as_ref().id()
    }
}
