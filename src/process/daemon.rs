use super::{ProcessOwner, ProcessReference, ThreadOwner};
use crate::{
    filesystem::DirectoryDescriptor,
    locks::Mutex,
    map::{Map, INVALID_ID},
};

static DAEMON_PROCESSES: Mutex<Map<ProcessReference>> = Mutex::new(Map::new());

pub fn create_process(
    entry: usize,
    context: usize,
    working_directory: Option<DirectoryDescriptor>,
) -> ThreadOwner {
    let new_process = ProcessOwner::new(None, working_directory);
    let mut daemon_processes = DAEMON_PROCESSES.lock();
    daemon_processes.insert(new_process.reference());
    new_process.create_thread(entry, context)
}

pub fn get_daemon_process(pid: isize) -> Option<ProcessReference> {
    DAEMON_PROCESSES
        .lock()
        .get(pid)
        .map(|reference| reference.clone())
}

pub fn kill_process(pid: isize) {
    let current_process = match super::get_current_thread().process() {
        Some(process) => process,
        None => return,
    };
    let mut session = DAEMON_PROCESSES.lock();

    unsafe {
        let critical_state = crate::critical::enter_local();

        let remove = match session.get(pid) {
            Some(process) => {
                if *process == current_process {
                    super::exit_process(128);
                } else {
                    process.kill_threads(INVALID_ID);
                    process.pre_exit(128);
                    true
                }
            }
            None => false,
        };

        if remove {
            session.remove(pid);
        }

        crate::critical::leave_local(critical_state);
    }
}
