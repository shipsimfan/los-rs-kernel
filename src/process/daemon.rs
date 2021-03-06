use super::{ProcessOwner, ProcessReference, ThreadOwner};
use crate::{
    filesystem::DirectoryDescriptor,
    ipc::Signals,
    locks::Mutex,
    map::{Map, INVALID_ID},
};
use alloc::{string::String, vec::Vec};

static DAEMON_PROCESSES: Mutex<Map<ProcessReference>> = Mutex::new(Map::new());

pub fn create_process(
    entry: usize,
    context: usize,
    working_directory: Option<DirectoryDescriptor>,
    name: String,
    signals: Signals,
) -> ThreadOwner {
    let new_process = ProcessOwner::new(None, working_directory, name, signals);
    let mut daemon_processes = DAEMON_PROCESSES.lock();
    daemon_processes.insert(new_process.reference());
    drop(daemon_processes);
    new_process.create_thread(entry, context, false)
}

pub fn get_daemon_process(pid: isize) -> Option<ProcessReference> {
    DAEMON_PROCESSES
        .lock()
        .get(pid)
        .map(|reference| reference.clone())
}

pub fn get_daemon_processes() -> Vec<isize> {
    DAEMON_PROCESSES.lock().ids()
}

pub fn kill_process(pid: isize) {
    let current_process = match super::get_current_thread().process() {
        Some(process) => process,
        None => return,
    };
    let mut session = DAEMON_PROCESSES.lock();

    unsafe {
        crate::critical::enter_local();

        let remove = match session.get(pid) {
            Some(process) => {
                if *process == current_process {
                    crate::critical::leave_local_without_sti();
                    super::exit_process(128);
                } else {
                    let threads = process.get_threads(INVALID_ID);
                    for thread in threads {
                        thread.clear_queue(false);
                    }
                    process.pre_exit(128);
                    true
                }
            }
            None => false,
        };

        if remove {
            session.remove(pid);
        }

        crate::critical::leave_local();
    }
}
