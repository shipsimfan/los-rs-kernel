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

static mut PROCESS_INITIALIZED: bool = false;

pub fn initialize<O: ProcessOwner<D, S> + 'static, D: 'static, S: Signals + 'static>() {
    unsafe {
        assert!(!PROCESS_INITIALIZED);
        PROCESS_INITIALIZED = true;

        execution::initialize::<O, D, S>();
    }
}
