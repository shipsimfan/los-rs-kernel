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

pub use directory::Directory;
pub use directory::ParentDirectory;
pub use directory_descriptor::DirectoryDescriptor;
pub use directory_entry::DirectoryEntry;
pub use file::File;
pub use file::FileMetadata;
pub use file_descriptor::FileDescriptor;
pub use file_descriptor::SeekFrom;

type DirectoryBox = directory::DirectoryBox;
type DirectoryContainer = directory::DirectoryContainer;
type FileBox = file::FileBox;
type FileContainer = file::FileContainer;
type DetectFilesystemFunction = filesystem::DetectFilesystemFunction;
type Filesystem = filesystem::Filesystem;
type FilesystemStarter = filesystem::FilesystemStarter;

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

pub fn open(filepath: &str) -> error::Result<FileDescriptor> {
    // Parse filepath
    let (fs_number, path) = parse_filepath(filepath, true)?;
    if path.len() == 0 {
        return Err(error::Status::InvalidArgument);
    }

    // Open filesystem
    let mut current_directory = match fs_number {
        Some(fs_number) => {
            let mut filesystems = FILESYSTEMS.lock();
            match filesystems.get_mut(fs_number) {
                Some(filesystem) => filesystem.root_directory().clone(),
                None => return Err(error::Status::NoFilesystem),
            }
        }
        None => {
            match process::get_current_thread_mut()
                .get_process_mut()
                .get_current_working_directory()
            {
                Some(dir) => dir.get_directory(),
                None => return Err(error::Status::NotSupported),
            }
        }
    };

    // Iterate path
    let mut iter = path.into_iter();
    let filename = match iter.next_back() {
        Some(str) => str,
        None => return Err(error::Status::InvalidArgument),
    };

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

    // Open file
    let file = current_directory
        .lock()
        .open_file(filename, &current_directory)?;

    // Create file descriptor
    Ok(FileDescriptor::new(file))
}

pub fn open_directory(path: &str) -> error::Result<DirectoryDescriptor> {
    // Parse filepath
    let (fs_number, path) = parse_filepath(path, false)?;

    // Open filesystem
    let mut current_directory = match fs_number {
        Some(fs_number) => {
            let mut filesystems = FILESYSTEMS.lock();
            match filesystems.get_mut(fs_number) {
                Some(filesystem) => filesystem.root_directory().clone(),
                None => return Err(error::Status::NoFilesystem),
            }
        }
        None => {
            match process::get_current_thread_mut()
                .get_process_mut()
                .get_current_working_directory()
            {
                Some(dir) => dir.get_directory(),
                None => return Err(error::Status::NotSupported),
            }
        }
    };

    // Iterate path
    for dir_name in path {
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

    Ok(DirectoryDescriptor::new(current_directory))
}

pub fn read(filepath: &str) -> error::Result<Vec<u8>> {
    // Parse filepath
    let (fs_number, path) = parse_filepath(filepath, true)?;

    // Open filesystem
    let mut current_directory = match fs_number {
        Some(fs_number) => {
            let mut filesystems = FILESYSTEMS.lock();
            match filesystems.get_mut(fs_number) {
                Some(filesystem) => filesystem.root_directory().clone(),
                None => return Err(error::Status::NoFilesystem),
            }
        }
        None => {
            match process::get_current_thread_mut()
                .get_process_mut()
                .get_current_working_directory()
            {
                Some(dir) => dir.get_directory(),
                None => return Err(error::Status::NotSupported),
            }
        }
    };

    // Iterate path
    let mut iter = path.into_iter();
    let filename = match iter.next_back() {
        Some(str) => str,
        None => return Err(error::Status::InvalidArgument),
    };

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

    // Get metadata and file
    let (metadata, file) = {
        let mut dir = current_directory.lock();
        let metadata = dir.get_file_metadata(filename)?;
        let file = dir.open_file(filename, &current_directory)?;
        (metadata, file)
    };

    // Create the buffer
    let mut buffer = Vec::with_capacity(metadata.size());
    unsafe { buffer.set_len(metadata.size()) };

    // Read the file
    let bytes_read = file.lock().read(0, buffer.as_mut_slice())?;
    unsafe { buffer.set_len(bytes_read) }; // Just in case

    Ok(buffer)
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
