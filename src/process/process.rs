use super::{Thread, ThreadQueue};
use crate::{
    error,
    filesystem::{self, DirectoryDescriptor, DirectoryEntry, FileDescriptor},
    map::{Map, Mappable, INVALID_ID},
    memory::AddressSpace,
    session::SessionBox,
};

pub struct Process {
    id: isize,
    threads: Map<Thread>,
    address_space: AddressSpace,
    session: Option<SessionBox>,
    exit_queue: ThreadQueue,
    file_descriptors: Map<FileDescriptor>,
    directory_descriptors: Map<DirectoryDescriptor>,
    current_working_directory: Option<DirectoryDescriptor>,
}

impl Process {
    pub fn new(
        session: Option<SessionBox>,
        current_working_directory: Option<DirectoryDescriptor>,
    ) -> Self {
        Process {
            id: INVALID_ID,
            threads: Map::new(),
            address_space: AddressSpace::new(),
            session,
            exit_queue: ThreadQueue::new(),
            file_descriptors: Map::new(),
            directory_descriptors: Map::new(),
            current_working_directory,
        }
    }

    pub fn create_thread(&mut self, entry: usize, context: usize) -> isize {
        let thread = Thread::new(self, entry, context);
        let tid = self.threads.insert(thread);
        super::queue_thread(self.threads.get_mut(tid).unwrap());
        tid
    }

    pub fn get_thread_mut(&mut self, tid: isize) -> Option<&mut Thread> {
        self.threads.get_mut(tid)
    }

    pub unsafe fn remove_thread(&mut self, tid: isize) -> bool {
        self.threads.remove(tid);
        self.threads.count() == 0
    }

    pub fn set_address_space_as_current(&self) {
        self.address_space.set_as_current()
    }

    pub fn get_session_mut(&mut self) -> Option<SessionBox> {
        match &self.session {
            Some(session) => Some(session.clone()),
            None => None,
        }
    }

    pub fn insert_into_exit_queue(&mut self, thread: &mut Thread) {
        self.exit_queue.push(thread);
    }

    pub unsafe fn kill_threads(&mut self, exception: isize) {
        self.threads.remove_all_but_one(exception);
    }

    pub fn pre_exit(&mut self, exit_status: isize) {
        if self.threads.count() != 1 {
            return;
        }

        while let Some(thread) = self.exit_queue.pop_mut() {
            thread.set_queue_data(exit_status);
            super::queue_thread(thread);
        }
    }

    pub fn open_file(&mut self, filepath: &str) -> Result<isize, error::Status> {
        let file_descriptor = filesystem::open(filepath)?;
        Ok(self.file_descriptors.insert(file_descriptor))
    }

    pub fn close_file(&mut self, fd: isize) {
        self.file_descriptors.remove(fd);
    }

    pub fn get_file(&mut self, fd: isize) -> Result<&mut FileDescriptor, error::Status> {
        match self.file_descriptors.get_mut(fd) {
            None => Err(error::Status::NotFound),
            Some(file_descriptor) => Ok(file_descriptor),
        }
    }

    pub fn get_current_working_directory(&mut self) -> Option<&mut DirectoryDescriptor> {
        match &mut self.current_working_directory {
            Some(dir) => Some(dir),
            None => None,
        }
    }

    pub fn set_current_working_directory(&mut self, directory: DirectoryDescriptor) {
        self.current_working_directory = Some(directory);
    }

    pub fn open_directory(&mut self, path: &str) -> Result<isize, error::Status> {
        let descriptor = filesystem::open_directory(path)?;
        Ok(self.directory_descriptors.insert(descriptor))
    }

    pub fn read_directory(&mut self, dd: isize) -> Result<Option<DirectoryEntry>, error::Status> {
        match self.directory_descriptors.get_mut(dd) {
            Some(descriptor) => Ok(descriptor.next()),
            None => Err(error::Status::NotFound),
        }
    }

    pub fn close_directory(&mut self, dd: isize) {
        self.directory_descriptors.remove(dd);
    }
}

impl Mappable for Process {
    fn id(&self) -> isize {
        self.id
    }

    fn set_id(&mut self, id: isize) {
        self.id = id
    }
}

impl Drop for Process {
    fn drop(&mut self) {
        if self.threads.count() > 0 {
            panic!("Dropping process while it still has threads!");
        }

        unsafe { self.address_space.free() };
    }
}
