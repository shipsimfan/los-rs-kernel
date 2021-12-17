use super::Process;
use crate::{
    filesystem::DirectoryDescriptor,
    locks::Mutex,
    map::{Map, INVALID_ID},
};

static DAEMON_PROCESSES: Mutex<Map<Process>> = Mutex::new(Map::new());

pub fn create_process(
    entry: usize,
    context: usize,
    working_directory: Option<DirectoryDescriptor>,
) -> isize {
    let new_process = Process::new(None, working_directory);
    let mut daemon_processes = DAEMON_PROCESSES.lock();
    let pid = daemon_processes.insert(new_process);
    daemon_processes
        .get_mut(pid)
        .unwrap()
        .create_thread(entry, context);
    pid
}

pub fn get_daemon() -> &'static Mutex<Map<Process>> {
    &DAEMON_PROCESSES
}

pub unsafe fn remove_process(pid: isize) {
    (*DAEMON_PROCESSES.as_ptr()).remove(pid)
}

pub fn kill_process(pid: isize) {
    let current_process = super::get_current_thread_mut().get_process_mut();
    let mut session = DAEMON_PROCESSES.lock();

    unsafe {
        crate::critical::enter_local();

        let remove = match session.get_mut(pid) {
            Some(process) => {
                if process as *const _ == current_process as *const _ {
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

        crate::critical::leave_local();
    }
}
