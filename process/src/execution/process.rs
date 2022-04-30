use super::{current_thread_option, thread_control};
use crate::{
    current_thread, exit_thread, kill_thread, queue_thread, yield_thread, Process, ProcessOwner,
    Signals, ThreadFunction,
};
use alloc::string::String;
use base::multi_owner::Reference;

pub fn create_process<O: ProcessOwner<D, S>, D, S: Signals>(
    entry: ThreadFunction,
    context: usize,
    descriptors: D,
    name: String,
    inherit_signals: bool,
) -> Reference<Process<O, D, S>> {
    // Get the process owner
    let (process_owner, signals) = match current_thread_option::<O, D, S>() {
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

pub fn wait_process<O: ProcessOwner<D, S> + 'static, D: 'static, S: Signals + 'static>(
    process: &Reference<Process<O, D, S>>,
) -> Option<isize> {
    match process.lock(|process| process.exit_queue()) {
        Some(queue) => {
            yield_thread(Some(queue));
            Some(
                current_thread::<O, D, S>()
                    .lock(|thread| thread.queue_data())
                    .unwrap(),
            )
        }
        None => None,
    }
}

pub fn kill_process<O: ProcessOwner<D, S> + 'static, D: 'static, S: Signals + 'static>(
    process: &Reference<Process<O, D, S>>,
    exit_status: isize,
) {
    unsafe { base::critical::enter_local() };

    if current_thread()
        .lock(|thread| thread.process().compare(process))
        .unwrap()
    {
        exit_process::<O, D, S>(exit_status);
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

pub fn exit_process<O: ProcessOwner<D, S> + 'static, D: 'static, S: Signals + 'static>(
    exit_status: isize,
) -> ! {
    unsafe { base::critical::enter_local() };

    let current_thread = current_thread::<O, D, S>();
    let current_process = current_thread.lock(|thread| thread.process()).unwrap();

    let threads = current_process
        .lock(|process| {
            process.set_exit_status(exit_status);
            process.threads()
        })
        .unwrap();

    for thread in threads.iter() {
        if !thread.compare(&current_thread) {
            kill_thread(thread, exit_status);
        }
    }

    unsafe { base::critical::leave_local_without_sti() };
    exit_thread::<O, D, S>(exit_status);
}
