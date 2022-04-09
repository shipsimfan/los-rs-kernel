use super::{
    inner::{Container, ProcessInfo, ProcessInner},
    ProcessOwner,
};
use crate::{
    conditional_variable::ConditionalVariable,
    critical::CriticalLock,
    error,
    filesystem::{DirectoryDescriptor, DirectoryEntry, FileDescriptor},
    ipc::{
        PipeReader, PipeWriter, SignalHandleReturn, SignalHandler, Signals, UserspaceSignalContext,
    },
    locks::Mutex,
    map::{Mappable, INVALID_ID},
    process::{CurrentQueue, ThreadOwner, ThreadReference},
    userspace_mutex::UserspaceMutex,
};
use alloc::{
    boxed::Box,
    sync::{Arc, Weak},
    vec::Vec,
};

#[derive(Clone)]
pub struct ProcessReference(Weak<CriticalLock<Box<ProcessInner>>>);

impl ProcessReference {
    pub fn new(process: Weak<CriticalLock<Box<ProcessInner>>>) -> Self {
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
        // Open directory first
        let directory_descriptor = crate::filesystem::open_directory(path, None)?;

        // Insert it into process
        match self.0.upgrade() {
            Some(process) => process.lock().open_directory(directory_descriptor),
            None => Err(crate::error::Status::NoProcess),
        }
    }

    pub fn read_directory(&self, dd: isize) -> crate::error::Result<Option<DirectoryEntry>> {
        let directory = match self.0.upgrade() {
            Some(process) => process.lock().get_directory(dd)?,
            None => return Err(crate::error::Status::NoProcess),
        };

        let ret = directory.lock().next();
        Ok(ret)
    }

    pub fn close_directory(&self, dd: isize) {
        match self.0.upgrade() {
            Some(process) => process.lock().close_directory(dd),
            None => {}
        }
    }

    pub fn clone_file(&self, descriptor: Container<FileDescriptor>) -> isize {
        match self.0.upgrade() {
            Some(process) => process.lock().clone_file(descriptor),
            None => INVALID_ID,
        }
    }

    pub fn open_file(&self, filepath: &str, flags: usize) -> crate::error::Result<isize> {
        // Open the file
        let file_descriptor = crate::filesystem::open(filepath, flags, None)?;

        // Insert into process
        match self.0.upgrade() {
            Some(process) => process.lock().open_file(file_descriptor),
            None => Err(crate::error::Status::NoProcess),
        }
    }

    pub fn close_file(&self, fd: isize) {
        match self.0.upgrade() {
            Some(process) => process.lock().close_file(fd),
            None => {}
        }
    }

    pub fn open_device(&self, path: &str) -> crate::error::Result<isize> {
        // Open device
        let device = crate::device::get_device(path)?;

        match self.0.upgrade() {
            Some(process) => process.lock().open_device(device),
            None => Err(crate::error::Status::NoProcess),
        }
    }

    pub fn close_device(&self, dd: isize) {
        match self.0.upgrade() {
            Some(process) => process.lock().close_device(dd),
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
            Some(process) => Some(process.create_thread(entry, context, false)),
            None => None,
        }
    }

    pub fn remove_thread(&self, tid: isize) -> bool {
        match self.0.upgrade() {
            Some(process) => process.lock().remove_thread(tid),
            None => true,
        }
    }

    pub fn get_threads(&self, exception: isize) -> Vec<ThreadReference> {
        match self.0.upgrade() {
            Some(process) => process.lock().get_threads(exception),
            None => Vec::new(),
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

    pub fn get_process_info(&self) -> Option<ProcessInfo> {
        match self.0.upgrade() {
            Some(process) => Some(process.lock().get_process_info()),
            None => None,
        }
    }

    pub fn signals(&self) -> Option<Signals> {
        match self.0.upgrade() {
            Some(process) => Some(process.lock().signals().clone()),
            None => None,
        }
    }

    pub fn raise(&self, signal: u8) {
        match self.0.upgrade() {
            Some(process) => process.lock().raise(signal),
            None => {}
        }
    }

    pub fn set_signal_handler(&self, signal: u8, handler: SignalHandler) {
        match self.0.upgrade() {
            Some(process) => process.lock().set_signal_handler(signal, handler),
            None => {}
        }
    }

    pub fn set_signal_mask(&self, signal: u8, mask: bool) {
        match self.0.upgrade() {
            Some(process) => process.lock().set_signal_mask(signal, mask),
            None => {}
        }
    }

    pub fn set_userspace_signal_handler(&self, handler: usize) {
        match self.0.upgrade() {
            Some(process) => process.lock().set_userspace_signal_handler(handler),
            None => {}
        }
    }

    pub fn handle_signals(
        &self,
        userspace_context: (UserspaceSignalContext, u64),
    ) -> SignalHandleReturn {
        match self.0.upgrade() {
            Some(process) => process.lock().handle_signals(userspace_context),
            None => SignalHandleReturn::None,
        }
    }

    pub fn create_mutex(&self) -> crate::error::Result<isize> {
        match self.0.upgrade() {
            Some(process) => Ok(process.lock().create_mutex()),
            None => Err(crate::error::Status::NoProcess),
        }
    }

    pub fn get_mutex(&self, md: isize) -> crate::error::Result<Arc<UserspaceMutex>> {
        match self.0.upgrade() {
            Some(process) => process.lock().get_mutex(md),
            None => Err(crate::error::Status::NoProcess),
        }
    }

    pub fn destroy_mutex(&self, md: isize) {
        match self.0.upgrade() {
            Some(process) => process.lock().destroy_mutex(md),
            None => {}
        }
    }

    pub fn create_cond_var(&self) -> crate::error::Result<isize> {
        match self.0.upgrade() {
            Some(process) => Ok(process.lock().create_cond_var()),
            None => Err(crate::error::Status::NoProcess),
        }
    }

    pub fn get_cond_var(&self, cond: isize) -> crate::error::Result<Arc<ConditionalVariable>> {
        match self.0.upgrade() {
            Some(process) => process.lock().get_cond_var(cond),
            None => Err(crate::error::Status::NoProcess),
        }
    }

    pub fn destroy_cond_var(&self, cond: isize) {
        match self.0.upgrade() {
            Some(process) => process.lock().destroy_cond_var(cond),
            None => {}
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

    /* Pipe funcs */
    pub fn create_pipe(&mut self) -> Option<(isize, isize)> {
        match self.0.upgrade() {
            Some(process) => Some(process.lock().create_pipe()),
            None => None,
        }
    }

    pub fn close_pipe_reader(&self, pr: isize) {
        match self.0.upgrade() {
            Some(process) => process.lock().close_pipe_reader(pr),
            None => {}
        }
    }

    pub fn close_pipe_writer(&self, pw: isize) {
        match self.0.upgrade() {
            Some(process) => process.lock().close_pipe_writer(pw),
            None => {}
        }
    }

    pub fn get_pipe_reader(&self, pr: isize) -> error::Result<Arc<Mutex<PipeReader>>> {
        match self.0.upgrade() {
            Some(process) => process.lock().get_pipe_reader(pr),
            None => Err(error::Status::BadDescriptor),
        }
    }

    pub fn get_pipe_writer(&self, pw: isize) -> error::Result<Arc<Mutex<PipeWriter>>> {
        match self.0.upgrade() {
            Some(process) => process.lock().get_pipe_writer(pw),
            None => Err(error::Status::BadDescriptor),
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
