#![no_std]

use alloc::boxed::Box;
use base::{
    map::{Map, Mappable},
    multi_owner::{Owner, Reference},
};
use descriptor::Descriptor;
use device::Device;
use filesystem::{DirectoryDescriptor, FileDescriptor, WorkingDirectory};
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
}

impl<T: process::ProcessTypes<Descriptor = Self> + 'static> WorkingDirectory<T> for Descriptors<T> {
    fn working_directory(&self) -> Option<&filesystem::DirectoryDescriptor<T>> {
        self.current_working_directory.as_ref()
    }
}
