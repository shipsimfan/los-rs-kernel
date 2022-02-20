use super::ProcessOwner;
use crate::{
    device::DeviceReference,
    error,
    filesystem::{DirectoryDescriptor, FileDescriptor},
    locks::{Mutex, MutexGuard},
    map::{Map, Mappable, INVALID_ID},
    memory::AddressSpace,
    process::{
        queue_thread,
        thread::{ThreadOwner, ThreadReference},
        CurrentQueue, ThreadQueue,
    },
    session::get_session_mut,
};
use alloc::sync::Arc;

#[derive(Clone)]
pub struct Container<T: Mappable>(Arc<Mutex<T>>);

pub struct DeviceDescriptor(DeviceReference, isize);

pub struct ProcessInner {
    id: isize,
    threads: Map<ThreadReference>,
    address_space: AddressSpace,
    session_id: Option<isize>,
    exit_queue: ThreadQueue,
    file_descriptors: Map<Container<FileDescriptor>>,
    directory_descriptors: Map<Container<DirectoryDescriptor>>,
    device_descriptors: Map<DeviceDescriptor>,
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
            device_descriptors: Map::new(),
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

    pub fn open_file(&mut self, file_descriptor: FileDescriptor) -> error::Result<isize> {
        Ok(self
            .file_descriptors
            .insert(Container::new(file_descriptor)))
    }

    pub fn close_file(&mut self, fd: isize) {
        self.file_descriptors.remove(fd);
    }

    pub fn get_file(&self, fd: isize) -> error::Result<Container<FileDescriptor>> {
        match self.file_descriptors.get(fd) {
            None => Err(error::Status::BadDescriptor),
            Some(file_descriptor) => Ok(file_descriptor.clone()),
        }
    }

    pub fn clone_file(&mut self, descriptor: Container<FileDescriptor>) -> isize {
        self.file_descriptors.insert(descriptor)
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

    pub fn open_directory(
        &mut self,
        directory_descriptor: DirectoryDescriptor,
    ) -> error::Result<isize> {
        Ok(self
            .directory_descriptors
            .insert(Container::new(directory_descriptor)))
    }

    pub fn get_directory(&self, dd: isize) -> error::Result<Container<DirectoryDescriptor>> {
        match self.directory_descriptors.get(dd) {
            Some(descriptor) => Ok(descriptor.clone()),
            None => Err(error::Status::BadDescriptor),
        }
    }

    pub fn close_directory(&mut self, dd: isize) {
        self.directory_descriptors.remove(dd);
    }

    pub fn open_device(&mut self, device: DeviceReference) -> error::Result<isize> {
        Ok(self
            .device_descriptors
            .insert(DeviceDescriptor(device, INVALID_ID)))
    }

    pub fn close_device(&mut self, dd: isize) {
        self.device_descriptors.remove(dd);
    }

    pub fn get_device(&self, dd: isize) -> error::Result<DeviceReference> {
        match self.device_descriptors.get(dd) {
            None => Err(error::Status::BadDescriptor),
            Some(device_descriptor) => Ok(device_descriptor.0.clone()),
        }
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

impl<T: Mappable> Container<T> {
    pub fn new(inner: T) -> Self {
        Container(Arc::new(Mutex::new(inner)))
    }

    pub fn lock(&self) -> MutexGuard<T> {
        self.0.lock()
    }
}

impl<T: Mappable> Mappable for Container<T> {
    fn id(&self) -> isize {
        self.0.lock().id()
    }

    fn set_id(&mut self, id: isize) {
        self.0.lock().set_id(id)
    }
}

impl Mappable for DeviceDescriptor {
    fn set_id(&mut self, id: isize) {
        self.1 = id;
    }

    fn id(&self) -> isize {
        self.1
    }
}