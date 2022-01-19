use crate::{
    device::{self, DeviceBox},
    error,
    locks::Mutex,
    map::Map,
    process,
};
use alloc::vec::Vec;
use core::ops::Deref;

mod directory;
mod directory_descriptor;
mod directory_entry;
pub mod drivers;
mod file;
mod file_descriptor;
mod filesystem;
mod metadata;

pub use directory::Directory;
pub use directory::Parent;
pub use directory_descriptor::DirectoryDescriptor;
pub use directory_entry::DirectoryEntry;
pub use file::File;
pub use file_descriptor::FileDescriptor;
pub use file_descriptor::SeekFrom;
pub use metadata::Metadata;

type DirectoryBox = directory::DirectoryBox;
type DirectoryContainer = directory::DirectoryContainer;
type FileBox = file::FileBox;
type FileContainer = file::FileContainer;
type DetectFilesystemFunction = filesystem::DetectFilesystemFunction;
type Filesystem = filesystem::Filesystem;
type FilesystemStarter = filesystem::FilesystemStarter;

const OPEN_READ: usize = 1;
const OPEN_WRITE: usize = 2;
const OPEN_READ_WRITE: usize = 3;
const OPEN_TRUNCATE: usize = 4;
const OPEN_APPEND: usize = 8;
const OPEN_CREATE: usize = 16;

static FILESYSTEM_DRIVERS: Mutex<Vec<DetectFilesystemFunction>> = Mutex::new(Vec::new());
static FILESYSTEMS: Mutex<Map<Filesystem>> = Mutex::new(Map::with_starting_index(1));

pub fn register_filesystem_driver(detect_function: DetectFilesystemFunction) {
    FILESYSTEM_DRIVERS.lock().push(detect_function);
}

pub fn register_drive(drive_path: &str) -> error::Result<()> {
    // Get the drive
    let drive_lock = device::get_device(drive_path)?;
    let mut drive = drive_lock.lock();

    // Get drive size
    let size = drive.ioctrl(0, 0)?;
    drop(drive);

    // Ignore zero size drives
    if size == 0 {
        return Ok(());
    }

    // TODO: Search for GUID Partition table

    // No GPT found, assuming whole disk is one partition
    detect_filesystem(drive_lock, 0, size)
}

pub fn open(filepath: &str, flags: usize) -> error::Result<FileDescriptor> {
    if flags & OPEN_READ_WRITE == 0 {
        return Err(error::Status::InvalidArgument);
    }

    // Parse filepath
    let (fs_number, path) = parse_filepath(filepath, true)?;
    if path.len() == 0 {
        return Err(error::Status::InvalidArgument);
    }

    // Open filesystem
    let root_directory = get_root_directory(fs_number)?;

    // Iterate path
    let (current_directory, filename) = get_directory(path, root_directory, true)?;
    let filename = match filename {
        Some(filename) => filename,
        None => return Err(error::Status::InvalidArgument),
    };

    // Open file
    let mut directory = current_directory.lock();
    let file = match directory.open_file(filename, &current_directory) {
        Ok(file) => file,
        Err(status) => match status {
            error::Status::NoEntry => {
                if flags & OPEN_CREATE != 0 {
                    directory.create_file(filename)?;
                    directory.open_file(filename, &current_directory)?
                } else {
                    return Err(status);
                }
            }
            _ => return Err(status),
        },
    };

    drop(directory);

    // Parse flags
    let read = flags & OPEN_READ != 0;
    let write = flags & OPEN_WRITE != 0;

    if flags & OPEN_TRUNCATE != 0 {
        file.lock().set_length(0)?;
    }

    let starting_offset = if flags & OPEN_APPEND != 0 {
        file.lock().get_length()
    } else {
        0
    };

    // Create file descriptor
    Ok(FileDescriptor::new(file, read, write, starting_offset))
}

pub fn open_directory(path: &str) -> error::Result<DirectoryDescriptor> {
    // Parse filepath
    let (fs_number, path) = parse_filepath(path, false)?;

    // Open filesystem
    let root_directory = get_root_directory(fs_number)?;

    // Iterate path
    let (directory, _) = get_directory(path, root_directory, false)?;

    Ok(DirectoryDescriptor::new(directory))
}

pub fn read(filepath: &str) -> error::Result<Vec<u8>> {
    // Parse filepath
    let (fs_number, path) = parse_filepath(filepath, true)?;

    // Open filesystem
    let root_directory = get_root_directory(fs_number)?;

    // Iterate path
    let (current_directory, filename) = get_directory(path, root_directory, true)?;
    let filename = match filename {
        Some(filename) => filename,
        None => return Err(error::Status::InvalidArgument),
    };

    // Get metadata and file
    let (metadata, file) = {
        let mut dir = current_directory.lock();
        let metadata = dir.get_metadata(filename)?;
        let file = dir.open_file(filename, &current_directory)?;
        (metadata, file)
    };

    // Create the buffer
    let mut buffer = Vec::with_capacity(metadata.size());
    unsafe { buffer.set_len(metadata.size()) };

    // Create the file descriptor
    let mut fd = FileDescriptor::new(file, true, false, 0);

    // Read the file
    let bytes_read = fd.read(buffer.as_mut_slice())?;
    if bytes_read > 0 {
        unsafe { buffer.set_len(bytes_read as usize) };
    } else {
        unsafe { buffer.set_len(0) };
    }

    Ok(buffer)
}

pub fn remove(path: &str) -> error::Result<()> {
    // Parse filepath
    let (fs_number, path) = parse_filepath(path, false)?;

    // Open filesystem
    let root_directory = get_root_directory(fs_number)?;

    // Iterate path
    let (parent_directory_lock, filename) = get_directory(path, root_directory, true)?;
    let filename = match filename {
        Some(str) => str,
        None => return Err(error::Status::InvalidArgument),
    };

    // Remove directory
    let mut parent_directory = parent_directory_lock.lock();
    parent_directory.remove(filename)
}

pub fn create_directory(path: &str) -> error::Result<()> {
    // Parse filepath
    let (fs_number, path) = parse_filepath(path, false)?;

    // Open filesystem
    let root_directory = get_root_directory(fs_number)?;

    // Iterate path
    let (parent_directory_lock, directory_name) = get_directory(path, root_directory, true)?;
    let directory_name = match directory_name {
        Some(str) => str,
        None => return Err(error::Status::InvalidArgument),
    };

    // Create directory
    let mut parent_directory = parent_directory_lock.lock();
    parent_directory.create_directory(directory_name)
}

fn detect_filesystem(drive: DeviceBox, start: usize, size: usize) -> error::Result<()> {
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

fn parse_filepath(filepath: &str, file: bool) -> error::Result<(Option<isize>, Vec<&str>)> {
    let filepath = if filepath.ends_with('/') {
        if file {
            return Err(error::Status::InvalidArgument);
        }
        &filepath[..filepath.len() - 1]
    } else {
        filepath
    };

    let mut iter = filepath.split(|c| -> bool { c == '\\' || c == '/' });

    // Parse drive number
    let drive_number = if filepath.starts_with(':') {
        match iter.next() {
            Some(str) => match isize::from_str_radix(&str[1..], 10) {
                Ok(value) => {
                    if value < 0 {
                        return Err(error::Status::InvalidArgument);
                    }
                    Some(value)
                }
                Err(_) => return Err(error::Status::InvalidArgument),
            },
            None => return Err(error::Status::InvalidArgument),
        }
    } else {
        None
    };

    // Get path
    let path = iter.collect();

    Ok((drive_number, path))
}

fn get_root_directory(fs_number: Option<isize>) -> error::Result<DirectoryBox> {
    match fs_number {
        Some(fs_number) => {
            let mut filesystems = FILESYSTEMS.lock();
            match filesystems.get_mut(fs_number) {
                Some(filesystem) => Ok(filesystem.root_directory().clone()),
                None => Err(error::Status::NoFilesystem),
            }
        }
        None => {
            let process_lock = process::get_current_thread()
                .process()
                .unwrap()
                .upgrade()
                .unwrap();
            let mut process = process_lock.lock();
            match process.current_working_directory() {
                Some(dir) => Ok(dir.get_directory()),
                None => Err(error::Status::NotSupported),
            }
        }
    }
}

fn get_directory(
    path: Vec<&str>,
    root_directory: DirectoryBox,
    filename: bool,
) -> error::Result<(DirectoryBox, Option<&str>)> {
    let mut iter = path.into_iter();
    let filename = if filename { iter.next_back() } else { None };

    let mut current_directory = root_directory;
    while let Some(dir_name) = iter.next() {
        if dir_name == "." {
            continue;
        }

        let mut directory = current_directory.lock();
        let new_directory = if dir_name == ".." {
            match directory.get_parent() {
                Some(parent) => parent,
                None => continue,
            }
        } else {
            directory.open_directory(dir_name, &current_directory)?
        };
        drop(directory);
        current_directory = new_directory;
    }

    Ok((current_directory, filename))
}
