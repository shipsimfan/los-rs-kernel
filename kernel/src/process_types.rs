use alloc::boxed::Box;
use base::map::Mappable;
use process::Signals;
use sessions::{DaemonSession, Session};

pub struct ProcessTypes;

impl process::ProcessTypes for ProcessTypes {
    type Owner = Box<dyn Session<Self>>;
    type Descriptor = TempDescriptors;
    type Signals = TempSignals;

    fn new_daemon() -> Self::Owner {
        let mut session = Box::new(DaemonSession::new());
        session.set_id(0);
        session
    }
}

pub struct TempDescriptors;
#[derive(Clone)]
pub struct TempSignals;

impl Signals for TempSignals {
    fn new() -> Self {
        TempSignals
    }
}
