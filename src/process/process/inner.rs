use super::ProcessOwner;
use crate::{
    error,
    filesystem::{self, DirectoryDescriptor, DirectoryEntry, FileDescriptor},
    map::{Map, Mappable, INVALID_ID},
    memory::AddressSpace,
    process::{
        queue_thread,
        thread::{ThreadOwner, ThreadReference},
        CurrentQueue, ThreadQueue,
    },
    session::get_session_mut,
};

pub struct ProcessInner {
    id: isize,
    threads: Map<ThreadReference>,
    address_space: AddressSpace,
    session_id: Option<isize>,
    exit_queue: ThreadQueue,
    file_descriptors: Map<FileDescriptor>,
    directory_descriptors: Map<DirectoryDescriptor>,
    current_working_directory: Option<DirectoryDescriptor>,
    process_time: isize,
}

impl ProcessInner {
    pub fn new(
        session_id: Option<isize>,
        current_working_directory: Option<DirectoryDescriptor>,
    ) -> Self {
        ProcessInner {
            id: INVALID_ID,
            threads: Map::new(),
            address_space: AddressSpace::new(),
            session_id,
            exit_queue: ThreadQueue::new(),
            file_descriptors: Map::new(),
            directory_descriptors: Map::new(),
            current_working_directory,
            process_time: 0,
        }
    }

    pub fn create_thread(
        &mut self,
        entry: usize,
        context: usize,
        self_owner: ProcessOwner,
    ) -> ThreadOwner {
        let thread = ThreadOwner::new(self_owner, entry, context);
        self.threads.insert(thread.reference());
        thread
    }

    pub fn get_thread(&self, tid: isize) -> Option<ThreadReference> {
        self.threads.get(tid).map(|reference| reference.clone())
    }

    pub fn remove_thread(&mut self, tid: isize) -> bool {
        self.threads.remove(tid);
        self.threads.count() == 0
    }

    pub fn set_address_space_as_current(&self) {
        self.address_space.set_as_current()
    }

    pub fn session_id(&mut self) -> Option<isize> {
        self.session_id
    }

    pub fn get_exit_queue(&self) -> CurrentQueue {
        self.exit_queue.into_current_queue()
    }

    pub fn kill_threads(&mut self, exception: isize) {
        self.threads.remove_all_except(exception);
    }

    pub unsafe fn pre_exit(&mut self, exit_status: isize) {
        if self.threads.count() > 1 {
            return;
        }

        while let Some(thread) = self.exit_queue.pop() {
            thread.set_queue_data(exit_status);
            queue_thread(thread);
        }
    }

    pub fn open_file(&mut self, filepath: &str, flags: usize) -> error::Result<isize> {
        let file_descriptor =
            filesystem::open(filepath, flags, self.current_working_directory.as_ref())?;
        Ok(self.file_descriptors.insert(file_descriptor))
    }

    pub fn close_file(&mut self, fd: isize) {
        self.file_descriptors.remove(fd);
    }

    pub fn get_file(&mut self, fd: isize) -> error::Result<&mut FileDescriptor> {
        match self.file_descriptors.get_mut(fd) {
            None => Err(error::Status::BadDescriptor),
            Some(file_descriptor) => Ok(file_descriptor),
        }
    }

    pub fn clone_file(&mut self, descriptor: &mut FileDescriptor) -> isize {
        self.file_descriptors.insert(descriptor.clone())
    }

    pub fn current_working_directory(&mut self) -> Option<&mut DirectoryDescriptor> {
        match &mut self.current_working_directory {
            Some(dir) => Some(dir),
            None => None,
        }
    }

    pub fn set_current_working_directory(&mut self, directory: DirectoryDescriptor) {
        self.current_working_directory = Some(directory);
    }

    pub fn open_directory(&mut self, path: &str) -> error::Result<isize> {
        let descriptor = filesystem::open_directory(path, self.current_working_directory.as_ref())?;
        Ok(self.directory_descriptors.insert(descriptor))
    }

    pub fn read_directory(&mut self, dd: isize) -> error::Result<Option<DirectoryEntry>> {
        match self.directory_descriptors.get_mut(dd) {
            Some(descriptor) => Ok(descriptor.next()),
            None => Err(error::Status::BadDescriptor),
        }
    }

    pub fn close_directory(&mut self, dd: isize) {
        self.directory_descriptors.remove(dd);
    }

    pub fn get_time(&self) -> isize {
        self.process_time
    }

    pub fn increase_time(&mut self, amount: isize) {
        self.process_time += amount;
    }
}

impl Mappable for ProcessInner {
    fn id(&self) -> isize {
        self.id
    }

    fn set_id(&mut self, id: isize) {
        self.id = id
    }
}

impl Drop for ProcessInner {
    fn drop(&mut self) {
        unsafe { self.address_space.free() };

        match self.session_id {
            Some(sid) => match get_session_mut(sid) {
                Some(session) => session.lock().remove_process(self.id),
                None => {}
            },
            None => {}
        }
    }
}
