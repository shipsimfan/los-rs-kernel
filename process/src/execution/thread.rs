use super::{current_thread, thread_control};
use crate::{CurrentQueue, Process, ProcessTypes, Thread, ThreadFunction};
use base::{
    critical::LOCAL_CRITICAL_COUNT,
    multi_owner::{Owner, Reference},
};
use core::{ffi::c_void, ptr::null};
use memory::KERNEL_VMA;

extern "C" {
    fn switch_stacks(save_location: *const usize, load_location: *const usize);
    fn thread_enter_user(context: usize, entry: *const c_void) -> !;
    fn set_fs_base(fs_base: usize);
}

pub fn create_thread<T: ProcessTypes + 'static>(
    entry: ThreadFunction,
    context: usize,
) -> Reference<Thread<T>> {
    create_thread_usize(entry as usize, context)
}

pub fn create_thread_usize<T: ProcessTypes + 'static>(
    entry: usize,
    context: usize,
) -> Reference<Thread<T>> {
    let current_process = current_thread().lock(|thread| thread.process().as_ref());

    let thread = Process::create_thread(current_process.upgrade(), entry, context);
    let ret = thread.as_ref();
    queue_thread(thread);
    ret
}

pub fn wait_thread<T: ProcessTypes + 'static>(thread: &Reference<Thread<T>>) -> Option<isize> {
    match thread.lock(|thread| thread.exit_queue()) {
        Some(queue) => {
            yield_thread(Some(queue));
            Some(current_thread::<T>().lock(|thread| thread.queue_data()))
        }
        None => None,
    }
}

pub fn queue_and_yield<T: ProcessTypes + 'static>() {
    let running_queue = thread_control::get::<T>().lock().running_queue();
    yield_thread(Some(running_queue))
}

pub fn queue_thread<T: ProcessTypes + 'static>(thread: Owner<Thread<T>>) {
    thread_control::get().lock().queue_execution(thread);
}

pub fn preempt<T: ProcessTypes + 'static>() {
    let tc = thread_control::get::<T>().lock();

    if !tc.is_next_thread() {
        return;
    }

    if tc.current_thread().is_none() {
        return;
    }

    drop(tc);

    queue_and_yield::<T>();
}

pub fn yield_thread<T: ProcessTypes + 'static>(queue: Option<CurrentQueue<T>>) {
    unsafe {
        assert!(LOCAL_CRITICAL_COUNT == 0);

        base::critical::enter_local();

        let control = thread_control::get::<T>();

        let mut tc = control.lock();

        let (next_thread, queue) = match tc.next_thread() {
            Some(thread) => (thread, queue),
            None => {
                match queue {
                    Some(queue) => queue.add(tc.current_thread().as_ref().unwrap().clone()),
                    None => {}
                }

                loop {
                    match tc.next_thread() {
                        Some(next_thread) => break (next_thread, None),
                        None => {}
                    }
                    drop(tc);
                    base::critical::leave_local();
                    assert!(LOCAL_CRITICAL_COUNT == 0);
                    core::arch::asm!("hlt");
                    base::critical::enter_local();
                    tc = control.lock();
                }
            }
        };

        // Access the current thread
        let default_location = 0;
        let current_stack_save_location = match tc.current_thread() {
            Some(current_thread) => current_thread.lock(|thread| {
                thread.save_float();
                thread.stack_pointer_location() as *const usize
            }),
            None => &default_location,
        };

        // Switch what we can now
        let new_stack_load_location = next_thread.lock(|thread| {
            thread
                .process()
                .lock(|process| process.set_address_space_as_current());
            thread.load_float();
            interrupts::set_interrupt_stack(thread.stack_top());
            set_fs_base(thread.tls_base());
            thread.stack_pointer_location() as *const usize
        });

        // Stage next thread
        tc.set_staged_thread(next_thread, queue);
        drop(tc);

        // Switch stacks
        switch_stacks(current_stack_save_location, new_stack_load_location);

        return post_yield::<T>(null(), 0);
    }
}

// This is here so both yield and thread_enter_kernel/user can access it
pub unsafe extern "C" fn post_yield<T: ProcessTypes + 'static>(
    entry: *const c_void,
    context: usize,
) {
    // Switch threads in the control
    let (old_thread, queue) = thread_control::get::<T>().lock().switch_staged_thread();

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
            thread_enter_kernel::<T>(entry, context)
        } else {
            thread_enter_user(context, entry)
        }
    } else {
        return;
    }
}

fn thread_enter_kernel<T: ProcessTypes + 'static>(entry: *const c_void, context: usize) {
    let entry: ThreadFunction = unsafe { core::mem::transmute(entry) };
    let status = entry(context);
    exit_thread::<T>(status);
}

pub fn kill_thread<T: ProcessTypes + 'static>(thread: &Reference<Thread<T>>, exit_status: isize) {
    unsafe {
        base::critical::enter_local();

        if thread.compare(&current_thread::<T>().as_ref()) {
            base::critical::leave_local_without_sti();
            exit_thread::<T>(exit_status);
        }

        thread.lock(|thread| {
            thread.set_exit_status(exit_status);
            thread.clear_queue(false).unwrap();
        });

        base::critical::leave_local();
    }
}

pub fn exit_thread<T: ProcessTypes + 'static>(exit_status: isize) -> ! {
    unsafe {
        base::critical::enter_local();

        let current_thread = current_thread::<T>();

        current_thread.lock(|thread| thread.set_exit_status(exit_status));

        base::critical::leave_local_without_sti();
        yield_thread::<T>(None);
        panic!("Returned to thread after exit!");
    }
}
