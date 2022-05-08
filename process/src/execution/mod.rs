use crate::{ProcessOwner, Signals, Thread, ThreadControl};
use alloc::boxed::Box;
use base::{critical::CriticalLock, multi_owner::Owner};
use core::{ffi::c_void, mem::ManuallyDrop, ptr::null};

mod process;
mod thread;

pub use process::*;
pub use thread::*;

type ThreadControlType<O, D, S> = &'static CriticalLock<ThreadControl<O, D, S>>;

// THREAD_CONTROL_PTR doesn't need a critical lock because it is set once at boot
static mut THREAD_CONTROL_PTR: *const c_void = null();

// CURRENT_THREAD_PTR sits inside THREAD_CONTROL_POINTER, this allows fast access without locking
// Accessing the current thread from the thread control without locking is fine.
static mut CURRENT_THREAD_PTR: *const c_void = null();

pub fn initialize<O: ProcessOwner<D, S> + 'static, D: 'static, S: Signals + 'static>() {
    unsafe {
        assert_eq!(THREAD_CONTROL_PTR, null());

        let thread_control = ManuallyDrop::new(Box::new(ThreadControl::<O, D, S>::new()));
        THREAD_CONTROL_PTR = thread_control.as_ref() as *const _ as *const _;
        CURRENT_THREAD_PTR = thread_control.lock().current_thread() as *const _ as *const _;
    }
}

// Used to get around rust not liking generics on statics
#[inline(always)]
fn thread_control<O: ProcessOwner<D, S> + 'static, D: 'static, S: Signals + 'static>(
) -> ThreadControlType<O, D, S> {
    unsafe { &*(THREAD_CONTROL_PTR as *const _) }
}

#[inline(always)]
fn cur_thread<O: ProcessOwner<D, S> + 'static, D: 'static, S: Signals + 'static>(
) -> &'static Option<Owner<Thread<O, D, S>>> {
    unsafe { &*(CURRENT_THREAD_PTR as *const _) }
}

#[inline(always)]
pub fn current_thread_option<O: ProcessOwner<D, S>, D, S: Signals>(
) -> Option<&'static Owner<Thread<O, D, S>>> {
    cur_thread().as_ref()
}

#[inline(always)]
pub fn current_thread<O: ProcessOwner<D, S>, D, S: Signals>() -> &'static Owner<Thread<O, D, S>> {
    current_thread_option().unwrap()
}
