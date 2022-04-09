use super::ProcessOwner;
use crate::{
    conditional_variable::ConditionalVariable,
    critical::CriticalLock,
    device::DeviceReference,
    error,
    filesystem::{DirectoryDescriptor, FileDescriptor},
    ipc::{
        Pipe, PipeReader, PipeWriter, SignalHandleReturn, SignalHandler, Signals,
        UserspaceSignalContext,
    },
    locks::{Mutex, MutexGuard},
    map::{Map, Mappable, MappedItem, INVALID_ID},
    memory::AddressSpace,
    process::{
        queue_thread,
        thread::{ThreadOwner, ThreadReference},
        CurrentQueue, ThreadQueue,
    },
    session::get_session,
    userspace_mutex::UserspaceMutex,
};
use alloc::{boxed::Box, string::String, sync::Arc, vec::Vec};

#[derive(Clone)]
pub struct Container<T>(Arc<Mutex<T>>);

#[derive(Clone)]
pub struct PipeWriterDescriptor(Arc<Mutex<PipeWriter>>, isize);
#[derive(Clone)]
pub struct PipeReaderDescriptor(Arc<Mutex<PipeReader>>, isize);

pub struct MutexDescriptor(Arc<UserspaceMutex>, isize);
pub struct CondVarDescriptor(Arc<ConditionalVariable>, isize);

pub struct ProcessInner {
    id: isize,
    threads: Map<ThreadReference>,
    address_space: AddressSpace,
    session_id: Option<isize>,
    exit_queue: ThreadQueue,
    file_descriptors: Map<MappedItem<Container<FileDescriptor>>>,
    directory_descriptors: Map<MappedItem<Container<DirectoryDescriptor>>>,
    device_descriptors: Map<MappedItem<DeviceReference>>,
    current_working_directory: Option<DirectoryDescriptor>,
    mutex_descriptors: Map<MutexDescriptor>,
    cond_var_descriptors: Map<CondVarDescriptor>,
    process_time: isize,
    name: String,
    signals: Signals,
    pipe_reader_descriptors: Map<PipeReaderDescriptor>,
    pipe_writer_descriptors: Map<PipeWriterDescriptor>,
}

pub struct ProcessInfo {
    pub num_threads: usize,
    pub time: usize,
    pub num_files: usize,
    pub num_directories: usize,
    pub working_directory: String,
    pub name: String,
}

impl ProcessInner {
    pub fn new(
        session_id: Option<isize>,
        current_working_directory: Option<DirectoryDescriptor>,
        name: String,
        signals: Signals,
    ) -> Arc<CriticalLock<Box<Self>>> {
        Arc::new(CriticalLock::new(Box::new(ProcessInner {
            id: INVALID_ID,
            threads: Map::new(),
            address_space: AddressSpace::new(),
            session_id,
            exit_queue: ThreadQueue::new(),
            file_descriptors: Map::new(),
            directory_descriptors: Map::new(),
            device_descriptors: Map::new(),
            mutex_descriptors: Map::new(),
            cond_var_descriptors: Map::new(),
            current_working_directory,
            process_time: 0,
            name,
            signals,
            pipe_reader_descriptors: Map::new(),
            pipe_writer_descriptors: Map::new(),
        })))
    }

    pub fn create_thread(
        &mut self,
        entry: usize,
        context: usize,
        self_owner: ProcessOwner,
        special: bool,
    ) -> ThreadOwner {
        let thread = ThreadOwner::new(self_owner, entry, context, special);
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

    pub fn get_threads(&mut self, exception: isize) -> Vec<ThreadReference> {
        let mut threads = Vec::with_capacity(self.threads.count());

        for thread in &self.threads {
            if thread.id() == exception {
                continue;
            }

            threads.push(thread.clone());
        }

        threads
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
            .insert(MappedItem::new(Container::new(file_descriptor))))
    }

    pub fn close_file(&mut self, fd: isize) {
        self.file_descriptors.remove(fd);
    }

    pub fn get_file(&self, fd: isize) -> error::Result<Container<FileDescriptor>> {
        match self.file_descriptors.get(fd) {
            None => Err(error::Status::BadDescriptor),
            Some(file_descriptor) => Ok((*file_descriptor).clone()),
        }
    }

    pub fn clone_file(&mut self, descriptor: Container<FileDescriptor>) -> isize {
        self.file_descriptors.insert(MappedItem::new(descriptor))
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
            .insert(MappedItem::new(Container::new(directory_descriptor))))
    }

    pub fn get_directory(&self, dd: isize) -> error::Result<Container<DirectoryDescriptor>> {
        match self.directory_descriptors.get(dd) {
            Some(descriptor) => Ok((*descriptor).clone()),
            None => Err(error::Status::BadDescriptor),
        }
    }

    pub fn close_directory(&mut self, dd: isize) {
        self.directory_descriptors.remove(dd);
    }

    pub fn open_device(&mut self, device: DeviceReference) -> error::Result<isize> {
        Ok(self.device_descriptors.insert(MappedItem::new(device)))
    }

    pub fn close_device(&mut self, dd: isize) {
        self.device_descriptors.remove(dd);
    }

    pub fn get_device(&self, dd: isize) -> error::Result<DeviceReference> {
        match self.device_descriptors.get(dd) {
            None => Err(error::Status::BadDescriptor),
            Some(device_descriptor) => Ok((*device_descriptor).clone()),
        }
    }

    pub fn create_mutex(&mut self) -> isize {
        self.mutex_descriptors
            .insert(MutexDescriptor(Arc::new(UserspaceMutex::new()), INVALID_ID))
    }

    pub fn get_mutex(&self, md: isize) -> error::Result<Arc<UserspaceMutex>> {
        match self.mutex_descriptors.get(md) {
            None => Err(error::Status::BadDescriptor),
            Some(mutex_descriptor) => Ok(mutex_descriptor.0.clone()),
        }
    }

    pub fn destroy_mutex(&mut self, md: isize) {
        self.mutex_descriptors.remove(md)
    }

    pub fn create_cond_var(&mut self) -> isize {
        self.cond_var_descriptors.insert(CondVarDescriptor(
            Arc::new(ConditionalVariable::new()),
            INVALID_ID,
        ))
    }

    pub fn get_cond_var(&self, cond: isize) -> error::Result<Arc<ConditionalVariable>> {
        match self.cond_var_descriptors.get(cond) {
            None => Err(error::Status::BadDescriptor),
            Some(cond_var_descriptor) => Ok(cond_var_descriptor.0.clone()),
        }
    }

    pub fn destroy_cond_var(&mut self, cond: isize) {
        self.cond_var_descriptors.remove(cond)
    }

    pub fn get_time(&self) -> isize {
        self.process_time
    }

    pub fn increase_time(&mut self, amount: isize) {
        self.process_time += amount;
    }

    pub fn get_process_info(&self) -> ProcessInfo {
        let working_directory = match &self.current_working_directory {
            Some(dir) => dir.get_full_path(),
            None => String::new(),
        };

        ProcessInfo {
            num_threads: self.threads.count(),
            time: self.process_time as usize,
            num_files: self.file_descriptors.count(),
            num_directories: self.directory_descriptors.count(),
            working_directory,
            name: self.name.clone(),
        }
    }

    pub fn signals(&self) -> &Signals {
        &self.signals
    }

    pub fn raise(&mut self, signal: u8) {
        self.signals.raise(signal);

        for thread in &self.threads {
            match thread.signal_interrupt() {
                Some(thread) => crate::process::queue_thread(thread),
                None => {}
            }
        }
    }

    pub fn set_signal_handler(&mut self, signal: u8, handler: SignalHandler) {
        self.signals.set_handler(signal, handler);
    }

    pub fn set_signal_mask(&mut self, signal: u8, mask: bool) {
        self.signals.mask(signal, mask);
    }

    pub fn set_userspace_signal_handler(&mut self, handler: usize) {
        self.signals.set_userspace_handler(handler)
    }

    pub fn handle_signals(
        &mut self,
        userspace_context: (UserspaceSignalContext, u64),
    ) -> SignalHandleReturn {
        self.signals.handle(userspace_context)
    }

    pub fn create_pipe(&mut self) -> (isize, isize) {
        let pipe = Pipe::new();

        //remove 4 subsequent lines when sharing pipes between procs? (do we still need to store prd and pwd in proc which creates the pipe?)
        let prd = PipeReaderDescriptor(Arc::new(Mutex::new(pipe.0)), INVALID_ID);
        let pwd = PipeWriterDescriptor(Arc::new(Mutex::new(pipe.1)), INVALID_ID);
        let prd = self.pipe_reader_descriptors.insert(prd.clone());
        let pwd = self.pipe_writer_descriptors.insert(pwd.clone());

        (prd, pwd)
    }

    pub fn get_pipe_reader(&self, pr: isize) -> error::Result<Arc<Mutex<PipeReader>>> {
        match self.pipe_reader_descriptors.get(pr) {
            None => Err(error::Status::BadDescriptor),
            Some(pr_descriptor) => Ok(pr_descriptor.0.clone()),
        }
    }
    pub fn get_pipe_writer(&self, pw: isize) -> error::Result<Arc<Mutex<PipeWriter>>> {
        match self.pipe_writer_descriptors.get(pw) {
            None => Err(error::Status::BadDescriptor),
            Some(pw_descriptor) => Ok(pw_descriptor.0.clone()),
        }
    }

    pub fn close_pipe_reader(&mut self, pr: isize) {
        self.pipe_reader_descriptors.remove(pr);
    }
    pub fn close_pipe_writer(&mut self, pw: isize) {
        self.pipe_writer_descriptors.remove(pw);
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
            Some(sid) => match get_session(sid) {
                Some(session) => session.lock().remove_process(self.id),
                None => {}
            },
            None => {}
        }
    }
}

impl<T> Container<T> {
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

impl Mappable for PipeReaderDescriptor {
    fn set_id(&mut self, id: isize) {
        self.1 = id;
    }

    fn id(&self) -> isize {
        self.1
    }
}

impl Mappable for PipeWriterDescriptor {
    fn set_id(&mut self, id: isize) {
        self.1 = id;
    }

    fn id(&self) -> isize {
        self.1
    }
}

impl Mappable for MutexDescriptor {
    fn set_id(&mut self, id: isize) {
        self.1 = id;
    }

    fn id(&self) -> isize {
        self.1
    }
}

impl Mappable for CondVarDescriptor {
    fn set_id(&mut self, id: isize) {
        self.1 = id;
    }

    fn id(&self) -> isize {
        self.1
    }
}
