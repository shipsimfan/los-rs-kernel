use crate::{ProcessOwner, Signals, Thread, ThreadControl};
use base::{critical::CriticalLock, multi_owner::Reference};
use core::{ffi::c_void, ptr::null};

mod process;
mod thread;

pub use process::*;
pub use thread::*;

type ThreadControlType<O, D, S> = &'static CriticalLock<ThreadControl<O, D, S>>;

// THREAD_CONTROL_PTR doesn't need a critical lock because it is set once at boot
static mut THREAD_CONTROL_PTR: *const c_void = null();

pub fn initialize<O: ProcessOwner<D, S>, D, S: Signals>(
    thread_control: ThreadControlType<O, D, S>,
) {
    unsafe {
        assert_eq!(THREAD_CONTROL_PTR, null());

        THREAD_CONTROL_PTR = thread_control as *const _ as *const _;
    }
}

// Used to get around rust not liking generics on statics
#[inline(always)]
fn thread_control<O: ProcessOwner<D, S> + 'static, D: 'static, S: Signals + 'static>(
) -> ThreadControlType<O, D, S> {
    unsafe { &*(THREAD_CONTROL_PTR as *const _) }
}

#[inline(always)]
fn current_thread_option<O: ProcessOwner<D, S>, D, S: Signals>(
) -> Option<Reference<Thread<O, D, S>>> {
    thread_control().lock().current_thread()
}

#[inline(always)]
fn current_thread<O: ProcessOwner<D, S>, D, S: Signals>() -> Reference<Thread<O, D, S>> {
    current_thread_option().unwrap()
}
