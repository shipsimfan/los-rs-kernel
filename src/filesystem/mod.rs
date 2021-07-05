use crate::{
    device::{self, DeviceBox},
    error,
    locks::Mutex,
    map::{Map, Mappable, INVALID_ID},
};
use alloc::{boxed::Box, string::String, sync::Arc, vec::Vec};
use core::ops::Deref;

pub mod drivers;

pub type DetectFilesystemFunction = fn(
    drive: DeviceBox,
    start: usize,
    size: usize,
) -> Result<Option<FilesystemStarter>, error::Status>;

type DirectoryBox = Arc<Mutex<DirectoryContainer>>;
type FileBox = Arc<Mutex<FileContainer>>;

pub trait Directory: Send {
    fn get_sub_files(&self) -> Result<Vec<String>, error::Status>; // Used to get sub files on initialization
    fn get_sub_directories(&self) -> Result<Vec<String>, error::Status>; // Used to get sub directories on initialization
    fn open_file(&self, filename: &str) -> Result<Box<dyn File>, error::Status>;
    fn open_directory(&self, directory_name: &str) -> Result<Box<dyn Directory>, error::Status>;
    fn make_file(&self, filename: &str) -> error::Result;
    fn make_directory(&self, directory_name: &str) -> error::Result;
    fn rename_file(&self, old_name: &str, new_name: &str) -> error::Result;
    fn rename_directory(&self, old_name: &str, new_name: &str) -> error::Result;
    fn remove_file(&self, filename: &str) -> error::Result;
    fn remove_directory(&self, directory_name: &str) -> error::Result;
}

pub trait File: Send {
    fn read(&mut self, offset: usize, buffer: &mut [u8]) -> error::Result;
    fn write(&mut self, offset: usize, buffer: &[u8]) -> error::Result;
    fn set_length(&mut self, new_length: usize) -> error::Result;
}

pub struct FilesystemStarter {
    volume_name: String,
    root_directory: Box<dyn Directory>,
}

struct DirectoryContainer {
    parent: Option<DirectoryBox>, // None for root directories
    directory: Box<dyn Directory>,
    sub_files: Vec<(String, Option<FileBox>)>,
    sub_directories: Vec<(String, Option<DirectoryBox>)>,
}

struct FileContainer {
    parent: FileBox,
    file: Box<dyn File>,
}

struct Filesystem {
    number: usize,
    _volume_name: String,
    root_directory: DirectoryContainer,
}

static FILESYSTEM_DRIVERS: Mutex<Vec<DetectFilesystemFunction>> = Mutex::new(Vec::new());
static FILESYSTEMS: Mutex<Map<Filesystem>> = Mutex::new(Map::with_starting_index(1));

pub fn register_filesystem_driver(detect_function: DetectFilesystemFunction) {
    FILESYSTEM_DRIVERS.lock().push(detect_function);
}

pub fn register_drive(drive_path: &str, size: usize) -> error::Result {
    if size == 0 {
        // Ignore zero size drives
        return Err(error::Status::InvalidArgument);
    }

    // Get the drive
    let drive = device::get_device(drive_path)?;

    // TODO: Search for GUID Partition table

    // No GPT found, assuming whole disk is one partition
    detect_filesystem(drive, 0, size)
}

fn detect_filesystem(drive: DeviceBox, start: usize, size: usize) -> error::Result {
    let drivers = FILESYSTEM_DRIVERS.lock();

    for filesystem in drivers.deref() {
        match filesystem(drive.clone(), start, size)? {
            Some(filesystem_starter) => register_filesystem(Filesystem::new(filesystem_starter)?),
            None => {}
        }
    }

    Ok(())
}

fn register_filesystem(filesystem: Filesystem) {
    FILESYSTEMS.lock().insert(filesystem);
}

impl DirectoryContainer {
    pub fn new(
        directory: Box<dyn Directory>,
        parent_directory: Option<DirectoryBox>,
    ) -> Result<Self, error::Status> {
        let sub_file_names = directory.get_sub_files()?;
        let mut sub_files = Vec::with_capacity(sub_file_names.len());
        for sub_file_name in sub_file_names {
            sub_files.push((sub_file_name, None));
        }

        let sub_directory_names = directory.get_sub_directories()?;
        let mut sub_directories = Vec::with_capacity(sub_directory_names.len());
        for sub_directory_name in sub_directory_names {
            sub_directories.push((sub_directory_name, None));
        }

        Ok(DirectoryContainer {
            parent: parent_directory,
            directory: directory,
            sub_files: sub_files,
            sub_directories: sub_directories,
        })
    }
}

impl Filesystem {
    pub fn new(filesystem_starter: FilesystemStarter) -> Result<Self, error::Status> {
        Ok(Filesystem {
            number: INVALID_ID,
            root_directory: DirectoryContainer::new(filesystem_starter.root_directory, None)?,
            _volume_name: filesystem_starter.volume_name,
        })
    }
}

impl Mappable for Filesystem {
    fn id(&self) -> usize {
        self.number
    }

    fn set_id(&mut self, id: usize) {
        self.number = id;
    }
}
