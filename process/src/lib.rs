#![no_std]
#![feature(const_fn_trait_bound)]

use alloc::string::String;
use base::{
    critical::{CriticalLock, LOCAL_CRITICAL_COUNT},
    log_info,
    multi_owner::{Owner, Reference},
};
use core::{ffi::c_void, ptr::null};

mod control;
mod process;
mod thread;
mod thread_queue;

extern crate alloc;

pub use control::ThreadControl;
pub use process::{Process, ProcessOwner, Signals};
pub use thread::{CurrentQueue, Thread, ThreadFunction};

static mut PROCESS_INITIALIZED: bool = false;

// THREAD_CONTROL_PTR doesn't need a critical lock because it is set once at boot
static mut THREAD_CONTROL_PTR: *const c_void = null();

const MODULE_NAME: &'static str = "Process Manager";

extern "C" {
    fn switch_stacks(save_location: *const usize, load_location: *const usize);
}

// UTIL FUNCTIONS

// Used to get around rust not liking generics on statics
#[inline(always)]
fn thread_control<O: ProcessOwner<D, S> + 'static, D: 'static, S: Signals + 'static>(
) -> &'static CriticalLock<ThreadControl<O, D, S>> {
    unsafe { &*(THREAD_CONTROL_PTR as *const _) }
}

#[inline(always)]
fn get_current_thread_option<O: ProcessOwner<D, S>, D, S: Signals>(
) -> Option<Reference<Thread<O, D, S>>> {
    thread_control().lock().get_current_thread()
}

#[inline(always)]
fn get_current_thread<O: ProcessOwner<D, S>, D, S: Signals>() -> Reference<Thread<O, D, S>> {
    get_current_thread_option().unwrap()
}

// INITIALIZE

pub fn initialize<O: ProcessOwner<D, S>, D, S: Signals>(
    thread_control: &'static CriticalLock<ThreadControl<O, D, S>>,
) {
    log_info!("Initializing . . .");

    unsafe {
        assert!(!PROCESS_INITIALIZED);
        PROCESS_INITIALIZED = true;

        THREAD_CONTROL_PTR = thread_control as *const _ as *const _;
    }

    log_info!("Initialized!");
}

// PROCESSES
pub fn create_process<O: ProcessOwner<D, S>, D, S: Signals>(
    entry: ThreadFunction,
    context: usize,
    descriptors: D,
    name: String,
    inherit_signals: bool,
) -> Reference<Process<O, D, S>> {
    // Get the process owner
    let (process_owner, signals) = match get_current_thread_option::<O, D, S>() {
        Some(current_thread) => current_thread
            .lock(|thread| {
                thread
                    .process()
                    .lock(|process| {
                        (
                            process.owner(),
                            if inherit_signals {
                                process.signals()
                            } else {
                                S::new()
                            },
                        )
                    })
                    .unwrap()
            })
            .unwrap(),
        None => (thread_control().lock().daemon_owner(), S::new()),
    };

    // Create a new process
    let new_process = Process::new(process_owner, descriptors, signals, name);
    let ret = new_process.as_ref();

    // Create the first thread
    let new_thread = Process::create_thread(new_process, entry, context);
    queue_thread(new_thread);

    ret
}

// THREADS
pub fn yield_thread<O: ProcessOwner<D, S> + 'static, D: 'static, S: Signals + 'static>(
    queue: Option<CurrentQueue<O, D, S>>,
) {
    unsafe {
        assert!(LOCAL_CRITICAL_COUNT == 0);

        loop {
            base::critical::enter_local();

            let mut tc = thread_control::<O, D, S>().lock();

            let next_thread = match tc.get_next_thread() {
                Some(thread) => thread,
                None => {
                    drop(tc);
                    base::critical::leave_local();
                    core::arch::asm!("hlt");
                    continue;
                }
            };

            // Access the current thread
            let default_location = 0;
            let current_stack_save_location = match tc.get_current_thread() {
                Some(current_thread) => current_thread
                    .lock(|thread| {
                        thread.save_float();
                        thread.stack_pointer_location() as *const usize
                    })
                    .unwrap(),
                None => &default_location,
            };

            // Switch what we can now
            let new_stack_load_location = next_thread.lock(|thread| {
                thread
                    .process()
                    .lock(|process| process.set_address_space_as_current());
                thread.load_float();
                interrupts::set_interrupt_stack(thread.stack_top());
                thread.stack_pointer_location() as *const usize
            });

            // Stage next thread
            tc.set_staged_thread(next_thread, queue);
            drop(tc);

            // Switch stacks
            switch_stacks(current_stack_save_location, new_stack_load_location);

            // Switch threads in the control
            let (old_thread, queue) = thread_control::<O, D, S>().lock().switch_staged_thread();

            // Insert the old thread or drop
            match old_thread {
                Some(old_thread) => match queue {
                    Some(queue) => queue.add(old_thread),
                    None => drop(old_thread),
                },
                None => {}
            }

            // Return
            base::critical::leave_local();
            return;
        }
    }
}

pub fn queue_thread<O: ProcessOwner<D, S> + 'static, D: 'static, S: Signals + 'static>(
    thread: Owner<Thread<O, D, S>>,
) {
    thread_control().lock().queue_execution(thread);
}

pub fn exit_thread<O: ProcessOwner<D, S> + 'static, D: 'static, S: Signals + 'static>(
    exit_status: isize,
    critical: bool,
) -> ! {
    unsafe {
        if !critical {
            base::critical::enter_local();
        }

        let current_thread = get_current_thread::<O, D, S>();

        current_thread.lock(|thread| thread.set_exit_status(exit_status));

        base::critical::leave_local_without_sti();
        yield_thread::<O, D, S>(None);
        panic!("Returned to thread after exit!");
    }
}
