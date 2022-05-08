use base::{map::Map, multi_owner::Reference};
use process::{Process, ProcessOwner, Signals};

pub struct ProcessTypes;

impl process::ProcessTypes for ProcessTypes {
    type Owner = TempSession<Self>;
    type Descriptor = TempDescriptors;
    type Signals = TempSignals;
}

pub struct TempSession<T: process::ProcessTypes + 'static>(Map<Reference<Process<T>>>);
pub struct TempDescriptors;
#[derive(Clone)]
pub struct TempSignals;

impl<T: process::ProcessTypes> ProcessOwner<T> for TempSession<T> {
    fn new_daemon() -> Self {
        TempSession(Map::new())
    }

    fn insert_process(&mut self, process: Reference<Process<T>>) {
        self.0.insert(process);
    }

    fn drop_process(&mut self, id: isize) {
        self.0.remove(id);
    }
}

impl Signals for TempSignals {
    fn new() -> Self {
        TempSignals
    }
}
