use base::{
    critical::CriticalLock,
    map::Map,
    multi_owner::{Owner, Reference},
};
use process::{Process, ProcessOwner, Signals, ThreadControl};

pub struct TempSession<D: 'static, S: 'static + Signals>(Map<Reference<Process<Self, D, S>>>);
pub struct TempDescriptors;
#[derive(Clone)]
pub struct TempSignals;

pub static mut THREAD_CONTROL: Option<
    CriticalLock<
        ThreadControl<TempSession<TempDescriptors, TempSignals>, TempDescriptors, TempSignals>,
    >,
> = None;

impl<D, S: Signals> TempSession<D, S> {
    pub fn new() -> Owner<Self> {
        Owner::new(TempSession(Map::new()))
    }
}

impl<D: 'static, S: 'static + Signals> ProcessOwner<D, S> for TempSession<D, S> {
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
