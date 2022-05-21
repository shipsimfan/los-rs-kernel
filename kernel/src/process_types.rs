use alloc::boxed::Box;
use base::map::Mappable;
use filesystem::{DirectoryDescriptor, WorkingDirectory};
use process::Signals;
use sessions::{DaemonSession, Session};

pub struct ProcessTypes;

pub struct Descriptors<T: process::ProcessTypes<Descriptor = Self> + 'static> {
    current_working_directory: Option<DirectoryDescriptor<T>>,
}

impl process::ProcessTypes for ProcessTypes {
    type Owner = Box<dyn Session<Self>>;
    type Descriptor = Descriptors<Self>;
    type Signals = TempSignals;

    fn new_daemon() -> Self::Owner {
        let mut session = Box::new(DaemonSession::new());
        session.set_id(0);
        session
    }
}

impl<T: process::ProcessTypes<Descriptor = Self> + 'static> Descriptors<T> {
    pub fn new(working_directory: Option<DirectoryDescriptor<T>>) -> Self {
        Descriptors {
            current_working_directory: working_directory,
        }
    }
}

impl<T: process::ProcessTypes<Descriptor = Self> + 'static> WorkingDirectory<T> for Descriptors<T> {
    fn working_directory(&self) -> Option<&filesystem::DirectoryDescriptor<T>> {
        self.current_working_directory.as_ref()
    }
}

#[derive(Clone)]
pub struct TempSignals;

impl Signals for TempSignals {
    fn new() -> Self {
        TempSignals
    }
}
