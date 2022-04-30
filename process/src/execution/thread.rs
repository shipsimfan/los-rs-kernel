use core::{ffi::c_void, ptr::null};

use super::{current_thread, thread_control};
use crate::{CurrentQueue, Process, ProcessOwner, Signals, Thread, ThreadFunction};
use base::{
    critical::LOCAL_CRITICAL_COUNT,
    multi_owner::{Owner, Reference},
};
use memory::KERNEL_VMA;

extern "C" {
    fn switch_stacks(save_location: *const usize, load_location: *const usize);
    fn thread_enter_user(context: usize, entry: *const c_void) -> !;
}

pub fn create_thread<O: ProcessOwner<D, S>, D, S: Signals>(
    entry: ThreadFunction,
    context: usize,
) -> Reference<Thread<O, D, S>> {
    let current_process = current_thread().lock(|thread| thread.process()).unwrap();

    let thread = Process::create_thread(current_process.upgrade(), entry, context);
    let ret = thread.as_ref();
    queue_thread(thread);
    ret
}

pub fn queue_and_yield<O: ProcessOwner<D, S> + 'static, D: 'static, S: Signals + 'static>() {
    let running_queue = thread_control::<O, D, S>().lock().running_queue();
    yield_thread(Some(running_queue))
}

pub fn queue_thread<O: ProcessOwner<D, S> + 'static, D: 'static, S: Signals + 'static>(
    thread: Owner<Thread<O, D, S>>,
) {
    thread_control().lock().queue_execution(thread);
}

pub fn yield_thread<O: ProcessOwner<D, S> + 'static, D: 'static, S: Signals + 'static>(
    queue: Option<CurrentQueue<O, D, S>>,
) {
    unsafe {
        assert!(LOCAL_CRITICAL_COUNT == 0);

        loop {
            base::critical::enter_local();

            let mut tc = thread_control::<O, D, S>().lock();

            let next_thread = match tc.next_thread() {
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
            let current_stack_save_location = match tc.current_thread() {
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

            return post_yield::<O, D, S>(null(), 0);
        }
    }
}

// This is here so both yield and thread_enter_kernel/user can access it
pub unsafe extern "C" fn post_yield<
    O: ProcessOwner<D, S> + 'static,
    D: 'static,
    S: Signals + 'static,
>(
    entry: *const c_void,
    context: usize,
) {
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

    if entry != null() {
        if entry as usize >= KERNEL_VMA {
            thread_enter_kernel::<O, D, S>(entry, context)
        } else {
            thread_enter_user(context, entry)
        }
    } else {
        return;
    }
}

fn thread_enter_kernel<O: ProcessOwner<D, S> + 'static, D: 'static, S: Signals + 'static>(
    entry: *const c_void,
    context: usize,
) {
    let entry: ThreadFunction = unsafe { core::mem::transmute(entry) };
    let status = entry(context);
    exit_thread::<O, D, S>(status);
}

pub fn kill_thread<O: ProcessOwner<D, S> + 'static, D: 'static, S: Signals + 'static>(
    thread: Reference<Thread<O, D, S>>,
    exit_status: isize,
) {
    unsafe {
        base::critical::enter_local();

        if thread.compare(&current_thread::<O, D, S>()) {
            base::critical::leave_local_without_sti();
            exit_thread::<O, D, S>(exit_status);
        }

        thread.lock(|thread| {
            thread.set_exit_status(exit_status);
            thread.clear_queue(false).unwrap();
        });
    }
}

pub fn exit_thread<O: ProcessOwner<D, S> + 'static, D: 'static, S: Signals + 'static>(
    exit_status: isize,
) -> ! {
    unsafe {
        base::critical::enter_local();

        let current_thread = current_thread::<O, D, S>();

        current_thread.lock(|thread| thread.set_exit_status(exit_status));

        base::critical::leave_local_without_sti();
        yield_thread::<O, D, S>(None);
        panic!("Returned to thread after exit!");
    }
}
