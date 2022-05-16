use crate::{ProcessTypes, Thread, ThreadControl};
use base::{log_info, multi_owner::Owner};
use core::{ffi::c_void, ptr::null};

mod process;
mod thread;

pub use process::*;
pub use thread::*;

crate::static_generic_local!(
    base::critical::CriticalLock<crate::ThreadControl<T>>,
    thread_control
);

// CURRENT_THREAD_PTR sits inside thread_control, this allows fast access without locking
// Accessing the current thread from the thread control without locking is fine.
static mut CURRENT_THREAD_PTR: *const c_void = null();

pub fn initialize<T: ProcessTypes + 'static>() {
    log_info!("Initializing thread control . . . ");

    thread_control::initialize(ThreadControl::<T>::new());

    unsafe {
        CURRENT_THREAD_PTR =
            thread_control::get::<T>().lock().current_thread() as *const _ as *const _;
    }

    log_info!("Initialized thread control!");
}

#[inline(always)]
pub fn current_thread_option<T: ProcessTypes>() -> Option<&'static Owner<Thread<T>>> {
    unsafe { &*(CURRENT_THREAD_PTR as *const Option<Owner<Thread<T>>>) }.as_ref()
}

#[inline(always)]
pub fn current_thread<T: ProcessTypes>() -> &'static Owner<Thread<T>> {
    current_thread_option().unwrap()
}
