use base::{map::Map, multi_owner::Reference};
use process::{Process, ProcessOwner, Signals};

pub struct TempSession<D: 'static, S: 'static + Signals>(Map<Reference<Process<Self, D, S>>>);
pub struct TempDescriptors;
#[derive(Clone)]
pub struct TempSignals;

impl<D: 'static, S: 'static + Signals> ProcessOwner<D, S> for TempSession<D, S> {
    fn new_daemon() -> Self {
        TempSession(Map::new())
    }

    fn insert_process(&mut self, process: Reference<Process<Self, D, S>>) {
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
