#![no_std]
#![feature(const_fn_trait_bound)]

mod control;
mod execution;
mod mutex;
mod process;
mod thread;
mod thread_queue;

extern crate alloc;

pub use control::ThreadControl;
pub use execution::{
    create_process, create_thread, current_thread, current_thread_option, exit_process,
    exit_thread, kill_process, kill_thread, queue_and_yield, queue_thread, wait_process,
    wait_thread, yield_thread,
};
pub use mutex::*;
pub use process::{Process, ProcessOwner, Signals};
pub use thread::{CurrentQueue, Thread, ThreadFunction};
pub use thread_queue::ThreadQueue;

pub trait ProcessTypes: Sized + Send {
    type Owner: ProcessOwner<Self>;
    type Descriptor;
    type Signals: process::Signals;

    fn new_daemon() -> Self::Owner;
}

static mut PROCESS_INITIALIZED: bool = false;

pub fn initialize<T: ProcessTypes + 'static>() {
    unsafe {
        assert!(!PROCESS_INITIALIZED);
        PROCESS_INITIALIZED = true;

        execution::initialize::<T>();
    }
}
