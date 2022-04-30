#![no_std]
#![feature(const_fn_trait_bound)]

use base::{critical::CriticalLock, log_info};

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

const MODULE_NAME: &'static str = "Process Manager";

pub fn initialize<O: ProcessOwner<D, S>, D, S: Signals>(
    thread_control: &'static CriticalLock<ThreadControl<O, D, S>>,
) {
    log_info!("Initializing . . .");

    unsafe {
        assert!(!PROCESS_INITIALIZED);
        PROCESS_INITIALIZED = true;

        execution::initialize(thread_control);
    }

    log_info!("Initialized!");
}
