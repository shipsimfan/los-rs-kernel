use super::{current_thread_option, thread_control};
use crate::{queue_thread, Process, ProcessOwner, Signals, ThreadFunction};
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
