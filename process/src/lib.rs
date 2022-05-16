#![no_std]

mod control;
mod execution;
mod mutex;
mod process;
mod thread;
mod thread_queue;

extern crate alloc;

use base::log_info;
pub use control::ThreadControl;
pub use execution::{
    create_process, create_thread, current_thread, current_thread_option, exit_process,
    exit_thread, kill_process, kill_thread, preempt, queue_and_yield, queue_thread, wait_process,
    wait_thread, yield_thread,
};
pub use mutex::*;
pub use process::{Process, ProcessOwner, Signals};
pub use thread::{CurrentQueue, Thread, ThreadFunction};
pub use thread_queue::{SortedThreadQueue, ThreadQueue};

#[macro_export]
macro_rules! __static_generic {
    ($type:ty, $name:ident, $prefix:ident) => {
        mod $name {
            static mut PTR: *const core::ffi::c_void = core::ptr::null();

            #[inline(always)]
            pub fn initialize<T: $prefix::ProcessTypes + 'static>(value: $type) {
                unsafe {
                    assert!(PTR == core::ptr::null());
                    let value = core::mem::ManuallyDrop::new(alloc::boxed::Box::new(value));
                    PTR = value.as_ref() as *const _ as *const _;
                }
            }

            #[inline(always)]
            pub fn get<T: $prefix::ProcessTypes + 'static>() -> &'static $type {
                unsafe { &*(PTR as *const _) }
            }
        }
    };
}

#[macro_export]
macro_rules! static_generic {
    ($type:ty, $name:ident) => {
        process::__static_generic!($type, $name, process);
    };
}

#[macro_export]
macro_rules! static_generic_local {
    ($type:ty, $name:ident) => {
        crate::__static_generic!($type, $name, crate);
    };
}

pub trait ProcessTypes: Sized + Send {
    type Owner: ProcessOwner<Self>;
    type Descriptor;
    type Signals: process::Signals;

    fn new_daemon() -> Self::Owner;
}

const MODULE_NAME: &str = "Process";

static mut PROCESS_INITIALIZED: bool = false;

pub fn initialize<T: ProcessTypes + 'static>() {
    log_info!("Initializing . . .");

    unsafe {
        assert!(!PROCESS_INITIALIZED);
        PROCESS_INITIALIZED = true;

        execution::initialize::<T>();
    }

    log_info!("Initialized!");
}
