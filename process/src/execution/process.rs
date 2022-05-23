use super::{current_thread_option, thread_control};
use crate::{
    current_thread, exit_thread, kill_thread, queue_thread, yield_thread, Process, ProcessTypes,
    Signals, ThreadFunction,
};
use alloc::string::String;
use base::multi_owner::{Owner, Reference};

pub fn create_process<T: ProcessTypes + 'static>(
    entry: ThreadFunction,
    context: usize,
    descriptors: T::Descriptor,
    name: String,
    inherit_signals: bool,
) -> Reference<Process<T>> {
    // Get the process owner
    let owner = match current_thread_option::<T>() {
        Some(current_thread) => {
            current_thread.lock(|thread| thread.process().lock(|process| process.owner().clone()))
        }
        None => thread_control::get::<T>().lock().daemon_owner().clone(),
    };

    create_process_owner(entry, context, descriptors, name, inherit_signals, owner)
}

pub fn create_process_owner<T: ProcessTypes + 'static>(
    entry: ThreadFunction,
    context: usize,
    descriptors: T::Descriptor,
    name: String,
    inherit_signals: bool,
    owner: Owner<T::Owner>,
) -> Reference<Process<T>> {
    // Determine signals
    let signals = if inherit_signals {
        match current_thread_option::<T>() {
            Some(current_thread) => current_thread
                .lock(|thread| thread.process().lock(|process| process.signals().clone())),
            None => T::Signals::new(),
        }
    } else {
        T::Signals::new()
    };

    // Create a new process
    let new_process = Process::new(owner, descriptors, signals, name);
    let ret = new_process.as_ref();

    // Create the first thread
    let new_thread = Process::create_thread(new_process, entry, context);
    queue_thread(new_thread);

    ret
}

pub fn wait_process<T: ProcessTypes + 'static>(process: &Reference<Process<T>>) -> Option<isize> {
    match process.lock(|process| process.exit_queue()) {
        Some(queue) => {
            yield_thread(Some(queue));
            Some(current_thread::<T>().lock(|thread| thread.queue_data()))
        }
        None => None,
    }
}

pub fn kill_process<T: ProcessTypes + 'static>(
    process: &Reference<Process<T>>,
    exit_status: isize,
) {
    unsafe { base::critical::enter_local() };

    if current_thread().lock(|thread| thread.process().compare_ref(&process)) {
        exit_process::<T>(exit_status);
    }

    let threads = match process.lock(|process| {
        process.set_exit_status(exit_status);
        process.threads()
    }) {
        Some(threads) => threads,
        None => {
            unsafe { base::critical::leave_local() };
            return;
        }
    };

    for thread in threads.iter() {
        kill_thread(thread, exit_status);
    }

    unsafe { base::critical::leave_local() };
}

pub fn exit_process<T: ProcessTypes + 'static>(exit_status: isize) -> ! {
    unsafe { base::critical::enter_local() };

    let threads = current_thread::<T>().lock(|thread| {
        thread.process().lock(|process| {
            process.set_exit_status(exit_status);
            process.threads()
        })
    });

    let current_thread = current_thread::<T>().as_ref();
    for thread in threads.iter() {
        if !thread.compare(&current_thread) {
            kill_thread(thread, exit_status);
        }
    }

    unsafe { base::critical::leave_local_without_sti() };
    exit_thread::<T>(exit_status);
}
