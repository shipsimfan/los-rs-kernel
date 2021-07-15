use super::{Thread, ThreadQueue};
use crate::{
    error,
    filesystem::{self, DirectoryDescriptor, FileDescriptor},
    logln,
    map::{Map, Mappable, INVALID_ID},
    memory::AddressSpace,
    session::Session,
};

pub struct Process {
    id: usize,
    threads: Map<Thread>,
    address_space: AddressSpace,
    session: Option<*mut Session>,
    exit_queue: ThreadQueue,
    file_descriptors: Map<FileDescriptor>,
    current_working_directory: Option<DirectoryDescriptor>,
}

impl Process {
    pub fn new(
        session: Option<&mut Session>,
        current_working_directory: Option<DirectoryDescriptor>,
    ) -> Self {
        Process {
            id: INVALID_ID,
            threads: Map::new(),
            address_space: AddressSpace::new(),
            session: match session {
                Some(session) => Some(session),
                None => None,
            },
            exit_queue: ThreadQueue::new(),
            file_descriptors: Map::new(),
            current_working_directory,
        }
    }

    pub fn create_thread(&mut self, entry: usize, context: usize) -> usize {
        let thread = Thread::new(self, entry, context);
        let tid = self.threads.insert(thread);
        super::queue_thread(self.threads.get_mut(tid).unwrap());
        tid
    }

    pub fn get_thread_mut(&mut self, tid: usize) -> Option<&mut Thread> {
        self.threads.get_mut(tid)
    }

    pub fn remove_thread(&mut self, id: usize) -> bool {
        self.threads.remove(id);
        self.threads.count() == 0
    }

    pub fn set_address_space_as_current(&self) {
        self.address_space.set_as_current()
    }

    pub fn get_session_mut(&mut self) -> Option<&mut Session> {
        match self.session {
            Some(session) => Some(unsafe { &mut *session }),
            None => None,
        }
    }

    pub fn insert_into_exit_queue(&mut self, thread: &mut Thread) {
        self.exit_queue.push(thread);
    }

    pub fn pre_exit(&mut self, exit_status: usize) {
        if self.threads.count() != 1 {
            return;
        }

        while let Some(thread) = self.exit_queue.pop_mut() {
            thread.set_queue_data(exit_status);
            super::queue_thread(thread);
        }
    }

    pub fn open_file(&mut self, filepath: &str) -> Result<usize, error::Status> {
        let file_descriptor = filesystem::open(filepath)?;
        Ok(self.file_descriptors.insert(file_descriptor))
    }

    pub fn close_file(&mut self, fd: usize) {
        self.file_descriptors.remove(fd);
    }

    pub fn get_file(&mut self, fd: usize) -> Result<&mut FileDescriptor, error::Status> {
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
}

impl Mappable for Process {
    fn id(&self) -> usize {
        self.id
    }

    fn set_id(&mut self, id: usize) {
        self.id = id
    }
}

impl Drop for Process {
    fn drop(&mut self) {
        if self.threads.count() > 0 {
            panic!("Dropping process while it still has threads!");
        }

        self.address_space.free();

        logln!("Dropping process!");
    }
}

unsafe impl Send for Process {}
