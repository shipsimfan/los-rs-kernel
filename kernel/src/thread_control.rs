use core::ffi::c_void;

use base::{map::Map, multi_owner::Reference};
use process::{exit_thread, Process, ProcessOwner, Signals, ThreadControl, ThreadFunction};

pub struct TempSession<D: 'static, S: 'static + Signals>(Map<Reference<Process<Self, D, S>>>);
pub struct TempDescriptors;
#[derive(Clone)]
pub struct TempSignals;

pub static mut THREAD_CONTROL: Option<
    ThreadControl<TempSession<TempDescriptors, TempSignals>, TempDescriptors, TempSignals>,
> = None;

#[no_mangle]
extern "C" fn thread_enter_kernel(entry: *const c_void, context: usize) {
    let entry: ThreadFunction = unsafe { core::mem::transmute(entry) };
    let status = entry(context);
    exit_thread::<TempSession<TempDescriptors, TempSignals>, TempDescriptors, TempSignals>(
        status, false,
    );
}

impl<D, S: Signals> TempSession<D, S> {
    pub fn new() -> Self {
        TempSession(Map::new())
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
