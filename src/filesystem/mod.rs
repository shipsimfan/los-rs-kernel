use crate::{
    device::{self, DeviceBox},
    error,
    locks::Mutex,
    map::Map,
};
use alloc::vec::Vec;
use core::ops::Deref;

mod directory;
pub mod drivers;
mod file;
mod file_descriptor;
mod filesystem;

pub use directory::Directory;
pub use file::File;
pub use file::FileMetadata;
pub use file_descriptor::FileDescriptor;

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

pub fn register_drive(drive_path: &str) -> error::Result {
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

pub fn open(filepath: &str) -> Result<FileDescriptor, error::Status> {
    // Parse filepath
    let (fs_number, path, filename) = parse_filepath(filepath)?;

    // Open filesystem
    let mut filesystems = FILESYSTEMS.lock();
    let filesystem = match filesystems.get_mut(fs_number) {
        Some(filesystem) => filesystem,
        None => return Err(error::Status::NotFound),
    };

    // Iterate path
    let mut current_directory = filesystem.root_directory().clone();
    for dir_name in path {
        let mut directory = current_directory.lock();
        let new_directory = directory.open_directory(dir_name, &current_directory)?;
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

pub fn read(filepath: &str) -> Result<Vec<u8>, error::Status> {
    // Parse filepath
    let (fs_number, path, filename) = parse_filepath(filepath)?;

    // Open filesystem
    let mut filesystems = FILESYSTEMS.lock();
    let filesystem = match filesystems.get_mut(fs_number) {
        Some(filesystem) => filesystem,
        None => return Err(error::Status::NotFound),
    };

    // Iterate path
    let mut current_directory = filesystem.root_directory().clone();
    for dir_name in path {
        let mut directory = current_directory.lock();
        let new_directory = directory.open_directory(dir_name, &current_directory)?;
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

fn parse_filepath(filepath: &str) -> Result<(usize, Vec<&str>, &str), error::Status> {
    let mut iter = filepath.split(|c| -> bool { c == '\\' || c == '/' });

    // Parse drive number
    let drive_number = match iter.next() {
        Some(str) => {
            if str.ends_with(':') {
                match usize::from_str_radix(&str[..str.len() - 1], 10) {
                    Ok(value) => value,
                    Err(_) => return Err(error::Status::InvalidArgument),
                }
            } else {
                return Err(error::Status::InvalidArgument);
            }
        }
        None => return Err(error::Status::InvalidArgument),
    };

    // Get filename
    let filename = match iter.next_back() {
        Some(value) => value,
        None => return Err(error::Status::InvalidArgument),
    };

    // Get path
    let path = iter.collect();

    Ok((drive_number, path, filename))
}
