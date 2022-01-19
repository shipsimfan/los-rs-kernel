use super::{inner::ProcessInner, ProcessOwner};
use crate::{
    filesystem::{DirectoryDescriptor, DirectoryEntry, FileDescriptor},
    locks::Spinlock,
    map::{Mappable, INVALID_ID},
    process::{CurrentQueue, ThreadOwner, ThreadReference},
};
use alloc::sync::Weak;

#[derive(Clone)]
pub struct ProcessReference(Weak<Spinlock<ProcessInner>>);

impl ProcessReference {
    pub fn new(process: Weak<Spinlock<ProcessInner>>) -> Self {
        ProcessReference(process)
    }

    pub fn session_id(&self) -> Option<isize> {
        match self.0.upgrade() {
            Some(process) => process.lock().session_id(),
            None => None,
        }
    }

    pub fn set_address_space_as_current(&self) {
        match self.0.upgrade() {
            Some(process) => process.lock().set_address_space_as_current(),
            None => {}
        }
    }

    pub fn set_current_working_directory(&self, directory: DirectoryDescriptor) {
        match self.0.upgrade() {
            Some(process) => process.lock().set_current_working_directory(directory),
            None => {}
        }
    }

    pub fn open_directory(&self, path: &str) -> crate::error::Result<isize> {
        match self.0.upgrade() {
            Some(process) => process.lock().open_directory(path),
            None => Err(crate::error::Status::NoProcess),
        }
    }

    pub fn read_directory(&self, dd: isize) -> crate::error::Result<Option<DirectoryEntry>> {
        match self.0.upgrade() {
            Some(process) => process.lock().read_directory(dd),
            None => Err(crate::error::Status::NoProcess),
        }
    }

    pub fn close_directory(&self, dd: isize) {
        match self.0.upgrade() {
            Some(process) => process.lock().close_directory(dd),
            None => {}
        }
    }

    pub fn clone_file(&self, descriptor: &mut FileDescriptor) -> isize {
        match self.0.upgrade() {
            Some(process) => process.lock().clone_file(descriptor),
            None => INVALID_ID,
        }
    }

    pub fn open_file(&self, filepath: &str, flags: usize) -> crate::error::Result<isize> {
        match self.0.upgrade() {
            Some(process) => process.lock().open_file(filepath, flags),
            None => Err(crate::error::Status::NoProcess),
        }
    }

    pub fn close_file(&self, fd: isize) {
        match self.0.upgrade() {
            Some(process) => process.lock().close_file(fd),
            None => {}
        }
    }

    pub fn get_exit_queue(&self) -> Option<CurrentQueue> {
        match self.0.upgrade() {
            Some(process) => Some(process.lock().get_exit_queue()),
            None => None,
        }
    }

    pub fn get_thread(&self, tid: isize) -> Option<ThreadReference> {
        match self.0.upgrade() {
            Some(process) => process.lock().get_thread(tid),
            None => None,
        }
    }

    pub fn create_thread(&self, entry: usize, context: usize) -> Option<ThreadOwner> {
        match self.upgrade() {
            Some(process) => Some(process.create_thread(entry, context)),
            None => None,
        }
    }

    pub fn remove_thread(&self, tid: isize) -> bool {
        match self.0.upgrade() {
            Some(process) => process.lock().remove_thread(tid),
            None => true,
        }
    }

    pub fn kill_threads(&self, exception: isize) {
        match self.0.upgrade() {
            Some(process) => process.lock().kill_threads(exception),
            None => {}
        }
    }

    pub fn increase_time(&self, amount: isize) {
        match self.0.upgrade() {
            Some(process) => process.lock().increase_time(amount),
            None => {}
        }
    }

    pub fn get_time(&self) -> isize {
        match self.0.upgrade() {
            Some(process) => process.lock().get_time(),
            None => 0,
        }
    }

    pub unsafe fn pre_exit(&self, exit_status: isize) {
        match self.0.upgrade() {
            Some(process) => process.lock().pre_exit(exit_status),
            None => {}
        }
    }

    pub fn upgrade(&self) -> Option<ProcessOwner> {
        match self.0.upgrade() {
            Some(process) => Some(ProcessOwner::new_raw(process)),
            None => None,
        }
    }
}

impl Mappable for ProcessReference {
    fn id(&self) -> isize {
        match self.0.upgrade() {
            Some(process) => process.lock().id(),
            None => INVALID_ID,
        }
    }

    fn set_id(&mut self, id: isize) {
        match self.0.upgrade() {
            Some(process) => process.lock().set_id(id),
            None => {}
        }
    }
}

impl PartialEq for ProcessReference {
    fn eq(&self, other: &Self) -> bool {
        match self.0.upgrade() {
            Some(us) => match other.0.upgrade() {
                Some(other) => us == other,
                None => false,
            },
            None => false,
        }
    }
}
