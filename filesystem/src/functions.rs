use crate::{Directory, DirectoryDescriptor, FileDescriptor, Parent};
use alloc::{boxed::Box, vec::Vec};
use base::{error::FILESYSTEM_MODULE_NUMBER, multi_owner::Owner};
use process::{Mutex, ProcessTypes};

pub trait WorkingDirectory<T: ProcessTypes<Descriptor = Self> + 'static> {
    fn working_directory(&self) -> Option<&DirectoryDescriptor<T>>;
}

#[derive(Debug)]
enum FilesystemError {
    InvalidUsageFlags,
    InvalidPath,
}

#[derive(Debug)]
enum RootDirectoryError {
    InvalidMountPoint,
    ProcessHasNoCurrentDirectory,
}

pub const OPEN_READ: usize = 1;
pub const OPEN_WRITE: usize = 2;
pub const OPEN_READ_WRITE: usize = 3;
pub const OPEN_TRUNCATE: usize = 4;
pub const OPEN_APPEND: usize = 8;
pub const OPEN_CREATE: usize = 16;

pub fn open<T: ProcessTypes<Descriptor: WorkingDirectory<T>> + 'static>(
    filepath: &str,
    flags: usize,
    starting_root: Option<&DirectoryDescriptor<T>>,
) -> base::error::Result<FileDescriptor<T>> {
    if flags & OPEN_READ_WRITE == 0 {
        return Err(Box::new(FilesystemError::InvalidUsageFlags));
    }

    // Parse filepath
    let (fs_number, path) = match parse_filepath(filepath, true) {
        Some(filepath) => filepath,
        None => return Err(Box::new(FilesystemError::InvalidPath)),
    };
    if path.len() == 0 {
        return Err(Box::new(FilesystemError::InvalidPath));
    }

    // Open filesystem
    let root_directory = get_root_directory(fs_number, starting_root)?;

    // Iterate path
    let (current_directory, filename) = get_directory(path, root_directory, true)?;
    let filename = match filename {
        Some(filename) => filename,
        None => return Err(Box::new(FilesystemError::InvalidPath)),
    };

    // Open file
    let file = current_directory.lock(|directory| match directory.open_file(filename) {
        Ok(file) => Ok(file),
        Err(error) => match error.error_number() {
            base::error::Status::NoEntry => {
                if flags & OPEN_CREATE != 0 {
                    directory.create_file(filename)?;
                    directory.open_file(filename)
                } else {
                    Err(error)
                }
            }
            _ => Err(error),
        },
    })?;

    // Parse flags
    let read = flags & OPEN_READ != 0;
    let write = flags & OPEN_WRITE != 0;

    if flags & OPEN_TRUNCATE != 0 {
        file.lock(|file| file.set_length(0))?;
    }

    let starting_offset = if flags & OPEN_APPEND != 0 {
        file.lock(|file| file.get_length())
    } else {
        0
    };

    // Create file descriptor
    Ok(FileDescriptor::new(file, read, write, starting_offset))
}

pub fn open_directory<T: ProcessTypes<Descriptor: WorkingDirectory<T>> + 'static>(
    path: &str,
    starting_root: Option<&DirectoryDescriptor<T>>,
) -> base::error::Result<DirectoryDescriptor<T>> {
    // Parse filepath
    let (fs_number, path) = match parse_filepath(path, false) {
        Some(path) => path,
        None => return Err(Box::new(FilesystemError::InvalidPath)),
    };

    // Open filesystem
    let root_directory = get_root_directory(fs_number, starting_root)?;

    // Iterate path
    let (directory, _) = get_directory(path, root_directory, false)?;

    Ok(DirectoryDescriptor::new(directory))
}

pub fn read<T: ProcessTypes<Descriptor: WorkingDirectory<T>> + 'static>(
    filepath: &str,
) -> base::error::Result<Vec<u8>> {
    // Parse filepath
    let (fs_number, path) = match parse_filepath(filepath, true) {
        Some(path) => path,
        None => return Err(Box::new(FilesystemError::InvalidPath)),
    };

    // Open filesystem
    let root_directory = get_root_directory::<T>(fs_number, None)?;

    // Iterate path
    let (current_directory, filename) = get_directory(path, root_directory, true)?;
    let filename = match filename {
        Some(filename) => filename,
        None => return Err(Box::new(FilesystemError::InvalidPath)),
    };

    // Get metadata and file
    let (metadata, file) =
        current_directory.lock(|directory| match directory.get_metadata(filename) {
            Ok(metadata) => match directory.open_file(filename) {
                Ok(file) => Ok((metadata, file)),
                Err(error) => Err(error),
            },
            Err(error) => Err(error),
        })?;

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

pub fn remove<T: ProcessTypes<Descriptor: WorkingDirectory<T>> + 'static>(
    path: &str,
) -> base::error::Result<()> {
    // Parse filepath
    let (fs_number, path) = match parse_filepath(path, false) {
        Some(path) => path,
        None => return Err(Box::new(FilesystemError::InvalidPath)),
    };

    // Open filesystem
    let root_directory = get_root_directory::<T>(fs_number, None)?;

    // Iterate path
    let (parent_directory_lock, filename) = get_directory(path, root_directory, true)?;
    let filename = match filename {
        Some(str) => str,
        None => return Err(Box::new(FilesystemError::InvalidPath)),
    };

    // Remove directory
    parent_directory_lock.lock(|parent_directory| parent_directory.remove(filename))
}

pub fn create_directory<T: ProcessTypes<Descriptor: WorkingDirectory<T>> + 'static>(
    path: &str,
) -> base::error::Result<()> {
    // Parse filepath
    let (fs_number, path) = match parse_filepath(path, false) {
        Some(path) => path,
        None => return Err(Box::new(FilesystemError::InvalidPath)),
    };

    // Open filesystem
    let root_directory = get_root_directory::<T>(fs_number, None)?;

    // Iterate path
    let (parent_directory_lock, directory_name) = get_directory(path, root_directory, true)?;
    let directory_name = match directory_name {
        Some(str) => str,
        None => return Err(Box::new(FilesystemError::InvalidPath)),
    };

    // Create directory
    parent_directory_lock.lock(|parent_directory| parent_directory.create_directory(directory_name))
}

fn parse_filepath(filepath: &str, file: bool) -> Option<(Option<usize>, Vec<&str>)> {
    let filepath = if filepath.ends_with('/') {
        if file {
            return None;
        }
        &filepath[..filepath.len() - 1]
    } else {
        filepath
    };

    let mut iter = filepath.split(|c| -> bool { c == '\\' || c == '/' });

    // Parse drive number
    let drive_number = if filepath.starts_with(':') {
        match iter.next() {
            Some(str) => match usize::from_str_radix(&str[1..], 10) {
                Ok(value) => Some(value),
                Err(_) => return None,
            },
            None => return None,
        }
    } else {
        None
    };

    // Get path
    let path = iter.collect();

    Some((drive_number, path))
}

fn get_root_directory<T: ProcessTypes<Descriptor: WorkingDirectory<T>> + 'static>(
    fs_number: Option<usize>,
    provided_root: Option<&DirectoryDescriptor<T>>,
) -> base::error::Result<Owner<Directory<T>, Mutex<Directory<T>, T>>> {
    match fs_number {
        Some(fs_number) => match crate::get_volume(fs_number) {
            Some(volume) => Ok(volume.lock(|volume| volume.root_directory().clone())),
            None => Err(Box::new(RootDirectoryError::InvalidMountPoint)),
        },
        None => match provided_root {
            Some(root_descriptor) => Ok(root_descriptor.get_directory().clone()),
            None => process::current_thread::<T>().lock(|thread| {
                thread
                    .process()
                    .lock(|process| match process.descriptors().working_directory() {
                        Some(dir) => Ok(dir.get_directory().clone()),
                        None => Err(RootDirectoryError::process_has_no_current_directory()),
                    })
            }),
        },
    }
}

fn get_directory<T: ProcessTypes + 'static>(
    path: Vec<&str>,
    root_directory: Owner<Directory<T>, Mutex<Directory<T>, T>>,
    filename: bool,
) -> base::error::Result<(Owner<Directory<T>, Mutex<Directory<T>, T>>, Option<&str>)> {
    let mut iter = path.into_iter();
    let filename = if filename { iter.next_back() } else { None };

    let mut current_directory = root_directory;
    while let Some(dir_name) = iter.next() {
        if dir_name == "." {
            continue;
        }

        let new_directory = if dir_name == ".." {
            match current_directory.lock(|directory| match directory.parent() {
                Parent::Directory(parent) => Some(parent.clone()),
                _ => None,
            }) {
                Some(parent) => parent,
                None => continue,
            }
        } else {
            current_directory.lock(|directory| directory.open_directory(dir_name))?
        };
        current_directory = new_directory;
    }

    Ok((current_directory, filename))
}

impl base::error::Error for FilesystemError {
    fn module_number(&self) -> i32 {
        FILESYSTEM_MODULE_NUMBER
    }

    fn error_number(&self) -> base::error::Status {
        base::error::Status::InvalidArgument
    }
}

impl core::fmt::Display for FilesystemError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            FilesystemError::InvalidPath => write!(f, "Invalid path"),
            FilesystemError::InvalidUsageFlags => write!(f, "Invalid usage flags"),
        }
    }
}

impl RootDirectoryError {
    pub fn process_has_no_current_directory() -> Box<dyn base::error::Error> {
        Box::new(RootDirectoryError::ProcessHasNoCurrentDirectory)
    }
}

impl base::error::Error for RootDirectoryError {
    fn module_number(&self) -> i32 {
        FILESYSTEM_MODULE_NUMBER
    }

    fn error_number(&self) -> base::error::Status {
        match self {
            RootDirectoryError::InvalidMountPoint => base::error::Status::InvalidArgument,
            RootDirectoryError::ProcessHasNoCurrentDirectory => base::error::Status::NotSupported,
        }
    }
}

impl core::fmt::Display for RootDirectoryError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            RootDirectoryError::InvalidMountPoint => write!(f, "Invalid mount point"),
            RootDirectoryError::ProcessHasNoCurrentDirectory => {
                write!(f, "Process has no current directory")
            }
        }
    }
}
