#![no_std]

use alloc::{boxed::Box, sync::Arc};
use base::{
    map::{Map, Mappable},
    multi_owner::{Owner, Reference},
};
use descriptor::Descriptor;
use device::Device;
use filesystem::{DirectoryDescriptor, FileDescriptor, WorkingDirectory};
use ipc::{ConditionalVariable, PipeReader, PipeWriter};
use process::Mutex;
use sessions::{DaemonSession, Session};

mod descriptor;

extern crate alloc;

pub struct ProcessTypes;

pub struct Descriptors<T: process::ProcessTypes<Descriptor = Self> + 'static> {
    current_working_directory: Option<DirectoryDescriptor<T>>,
    files: Map<Descriptor<Owner<FileDescriptor<T>>>>,
    directories: Map<Descriptor<Owner<DirectoryDescriptor<T>>>>,
    devices: Map<Descriptor<Reference<Box<dyn Device>, Mutex<Box<dyn Device>, T>>>>,
    pipe_readers: Map<Descriptor<PipeReader<T>>>,
    pipe_writers: Map<Descriptor<PipeWriter<T>>>,
    mutexes: Map<Descriptor<Arc<ipc::Mutex<T>>>>,
    conditional_variables: Map<Descriptor<Arc<ConditionalVariable<T>>>>,
}

impl process::ProcessTypes for ProcessTypes {
    type Owner = Box<dyn Session<Self>>;
    type Descriptor = Descriptors<Self>;
    type Signals = ipc::Signals;

    fn new_daemon() -> Self::Owner {
        let mut session = Box::new(DaemonSession::new());
        session.set_id(0);
        session
    }
}

impl<T: process::ProcessTypes<Descriptor = Self> + 'static> Descriptors<T> {
    pub fn new(working_directory: Option<DirectoryDescriptor<T>>) -> Self {
        Descriptors {
            current_working_directory: working_directory,
            files: Map::new(),
            directories: Map::new(),
            devices: Map::new(),
            pipe_readers: Map::new(),
            pipe_writers: Map::new(),
            mutexes: Map::new(),
            conditional_variables: Map::new(),
        }
    }

    pub fn file(&self, id: isize) -> Option<&Owner<FileDescriptor<T>>> {
        self.files.get(id).map(|descriptor| &**descriptor)
    }

    pub fn directory(&self, id: isize) -> Option<&Owner<DirectoryDescriptor<T>>> {
        self.directories.get(id).map(|descriptor| &**descriptor)
    }

    pub fn device(
        &self,
        id: isize,
    ) -> Option<&Reference<Box<dyn Device>, Mutex<Box<dyn Device>, T>>> {
        self.devices.get(id).map(|descriptor| &**descriptor)
    }

    pub fn pipe_reader(&self, id: isize) -> Option<&PipeReader<T>> {
        self.pipe_readers.get(id).map(|descriptor| &**descriptor)
    }

    pub fn pipe_writer(&self, id: isize) -> Option<&PipeWriter<T>> {
        self.pipe_writers.get(id).map(|descriptor| &**descriptor)
    }

    pub fn mutex(&self, id: isize) -> Option<&Arc<ipc::Mutex<T>>> {
        self.mutexes.get(id).map(|descriptor| &**descriptor)
    }

    pub fn conditional_variable(&self, id: isize) -> Option<&Arc<ConditionalVariable<T>>> {
        self.conditional_variables
            .get(id)
            .map(|descriptor| &**descriptor)
    }

    pub fn file_count(&self) -> usize {
        self.files.len()
    }

    pub fn directory_count(&self) -> usize {
        self.directories.len()
    }

    pub fn device_count(&self) -> usize {
        self.devices.len()
    }

    pub fn pipe_reader_count(&self) -> usize {
        self.pipe_readers.len()
    }

    pub fn pipe_writer_count(&self) -> usize {
        self.pipe_writers.len()
    }

    pub fn mutex_count(&self) -> usize {
        self.mutexes.len()
    }

    pub fn conditional_variable_count(&self) -> usize {
        self.conditional_variables.len()
    }

    pub fn set_working_directory(&mut self, directory: DirectoryDescriptor<T>) {
        self.current_working_directory = Some(directory);
    }

    pub fn insert_file(&mut self, file: FileDescriptor<T>) -> isize {
        self.files.insert(Descriptor::new(Owner::new(file)))
    }

    pub fn insert_directory(&mut self, directory: DirectoryDescriptor<T>) -> isize {
        self.directories
            .insert(Descriptor::new(Owner::new(directory)))
    }

    pub fn insert_device(
        &mut self,
        device: Reference<Box<dyn Device>, Mutex<Box<dyn Device>, T>>,
    ) -> isize {
        self.devices.insert(Descriptor::new(device))
    }

    pub fn insert_pipe_reader(&mut self, pipe_reader: PipeReader<T>) -> isize {
        self.pipe_readers.insert(Descriptor::new(pipe_reader))
    }

    pub fn insert_pipe_writer(&mut self, pipe_writer: PipeWriter<T>) -> isize {
        self.pipe_writers.insert(Descriptor::new(pipe_writer))
    }

    pub fn insert_mutex(&mut self, mutex: ipc::Mutex<T>) -> isize {
        self.mutexes.insert(Descriptor::new(Arc::new(mutex)))
    }

    pub fn insert_conditional_variable(
        &mut self,
        conditional_variable: ConditionalVariable<T>,
    ) -> isize {
        self.conditional_variables
            .insert(Descriptor::new(Arc::new(conditional_variable)))
    }

    pub fn remove_file(&mut self, id: isize) {
        self.files.remove(id);
    }

    pub fn remove_directory(&mut self, id: isize) {
        self.directories.remove(id);
    }

    pub fn remove_device(&mut self, id: isize) {
        self.devices.remove(id);
    }

    pub fn remove_pipe_reader(&mut self, id: isize) {
        self.pipe_readers.remove(id);
    }

    pub fn remove_pipe_writer(&mut self, id: isize) {
        self.pipe_writers.remove(id);
    }

    pub fn remove_mutex(&mut self, id: isize) {
        self.mutexes.remove(id);
    }

    pub fn remove_conditional_variable(&mut self, id: isize) {
        self.conditional_variables.remove(id);
    }
}

impl<T: process::ProcessTypes<Descriptor = Self> + 'static> WorkingDirectory<T> for Descriptors<T> {
    fn working_directory(&self) -> Option<&filesystem::DirectoryDescriptor<T>> {
        self.current_working_directory.as_ref()
    }
}
