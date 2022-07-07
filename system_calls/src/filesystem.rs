use crate::SystemCallError;
use alloc::boxed::Box;
use base::{error::SYSTEM_CALLS_MODULE_NUMBER, log_error};
use filesystem::SeekFrom;
use process_types::ProcessTypes;

#[derive(Debug)]
enum FilesystemError {
    NotFound,
}

const OPEN_FILE_SYSCALL: usize = 0x2000;
const CLOSE_FILE_SYSCALL: usize = 0x2001;
const SEEK_FILE_SYSCALL: usize = 0x2002;
const READ_FILE_SYSCALL: usize = 0x2003;
const OPEN_DIRECTORY_SYSCALL: usize = 0x2004;
const CLOSE_DIRECTORY_SYSCALL: usize = 0x2005;
const READ_DIRECTORY_SYSCALL: usize = 0x2006;
const TRUNCATE_FILE_SYSCALL: usize = 0x2007;
const WRITE_FILE_SYSCALL: usize = 0x2008;
const REMOVE_FILE_SYSCALL: usize = 0x2009;
const REMOVE_DIRECTORY_SYSCALL: usize = 0x200A;
const CREATE_DIRECTORY_SYSCALL: usize = 0x200B;
const TELL_FILE_SYSCALL: usize = 0x200C;

pub fn system_call(
    code: usize,
    arg1: usize,
    arg2: usize,
    arg3: usize,
    _arg4: usize,
    _arg5: usize,
) -> base::error::Result<isize> {
    match code {
        OPEN_FILE_SYSCALL => {
            let filepath = super::to_str(arg1)?;
            let file = filesystem::open::<ProcessTypes>(filepath, arg2, None)?;
            Ok(process::current_thread::<ProcessTypes>().lock(|thread| {
                thread
                    .process()
                    .lock(|process| process.descriptors_mut().insert_file(file))
            }))
        }
        CLOSE_FILE_SYSCALL => {
            let file = process::current_thread::<ProcessTypes>().lock(|thread| {
                thread
                    .process()
                    .lock(|process| process.descriptors_mut().remove_file(arg1 as isize))
            });
            drop(file);
            Ok(0)
        }
        SEEK_FILE_SYSCALL => {
            let file = match process::current_thread::<ProcessTypes>().lock(|thread| {
                thread.process().lock(|process| {
                    process
                        .descriptors()
                        .file(arg1 as isize)
                        .map(|file| file.clone())
                })
            }) {
                Some(file) => file,
                None => return Err(Box::new(FilesystemError::NotFound)),
            };
            Ok(file.lock(|file| (file.seek(arg2, SeekFrom::from(arg3)) & 0x7FFFFFFFFFFF) as isize))
        }
        READ_FILE_SYSCALL => {
            let file = match process::current_thread::<ProcessTypes>().lock(|thread| {
                thread.process().lock(|process| {
                    process
                        .descriptors()
                        .file(arg1 as isize)
                        .map(|file| file.clone())
                })
            }) {
                Some(file) => file,
                None => return Err(Box::new(FilesystemError::NotFound)),
            };

            let buffer = super::to_slice_mut(arg2, arg3)?;

            file.lock(|file| file.read(buffer))
        }
        OPEN_DIRECTORY_SYSCALL => {
            let path = super::to_str(arg1)?;
            let directory = filesystem::open_directory::<ProcessTypes>(path, None)?;
            Ok(process::current_thread::<ProcessTypes>().lock(|thread| {
                thread
                    .process()
                    .lock(|process| process.descriptors_mut().insert_directory(directory))
            }))
        }
        CLOSE_DIRECTORY_SYSCALL => {
            let directory = process::current_thread::<ProcessTypes>().lock(|thread| {
                thread
                    .process()
                    .lock(|process| process.descriptors_mut().remove_directory(arg1 as isize))
            });
            drop(directory);
            Ok(0)
        }
        READ_DIRECTORY_SYSCALL => {
            let desintation = super::to_ptr_mut(arg2)?;
            match process::current_thread::<ProcessTypes>().lock(|thread| {
                thread.process().lock(|process| {
                    process
                        .descriptors()
                        .directory(arg1 as isize)
                        .map(|directory| directory.clone())
                })
            }) {
                Some(directory) => Ok(match directory.lock(|directory| directory.next()) {
                    Some(dirent) => {
                        unsafe { *desintation = dirent };
                        1
                    }
                    None => 0,
                }),
                None => Err(Box::new(FilesystemError::NotFound)),
            }
        }
        TRUNCATE_FILE_SYSCALL => {
            let file = match process::current_thread::<ProcessTypes>().lock(|thread| {
                thread.process().lock(|process| {
                    process
                        .descriptors()
                        .file(arg1 as isize)
                        .map(|file| file.clone())
                })
            }) {
                Some(file) => file,
                None => return Err(Box::new(FilesystemError::NotFound)),
            };

            file.lock(|file| file.set_length(arg2))?;

            Ok(0)
        }
        WRITE_FILE_SYSCALL => {
            let file = match process::current_thread::<ProcessTypes>().lock(|thread| {
                thread.process().lock(|process| {
                    process
                        .descriptors()
                        .file(arg1 as isize)
                        .map(|file| file.clone())
                })
            }) {
                Some(file) => file,
                None => return Err(Box::new(FilesystemError::NotFound)),
            };

            let buffer = super::to_slice_mut(arg2, arg3)?;

            file.lock(|file| file.write(buffer))
        }
        REMOVE_FILE_SYSCALL => {
            let path = super::to_str(arg1)?;

            filesystem::remove::<ProcessTypes>(path)?;

            Ok(0)
        }
        REMOVE_DIRECTORY_SYSCALL => {
            let path = super::to_str(arg1)?;

            filesystem::remove::<ProcessTypes>(path)?;

            Ok(0)
        }
        CREATE_DIRECTORY_SYSCALL => {
            let path = super::to_str(arg1)?;

            filesystem::create_directory::<ProcessTypes>(path)?;

            Ok(0)
        }
        TELL_FILE_SYSCALL => {
            let file = match process::current_thread::<ProcessTypes>().lock(|thread| {
                thread.process().lock(|process| {
                    process
                        .descriptors()
                        .file(arg1 as isize)
                        .map(|file| file.clone())
                })
            }) {
                Some(file) => file,
                None => return Err(Box::new(FilesystemError::NotFound)),
            };

            Ok(file.lock(|file| file.tell()) as isize)
        }
        _ => {
            log_error!("Invalid filesystem system call: {}", code);
            Err(Box::new(SystemCallError::InvalidCode))
        }
    }
}

impl base::error::Error for FilesystemError {
    fn module_number(&self) -> i32 {
        SYSTEM_CALLS_MODULE_NUMBER
    }

    fn error_number(&self) -> base::error::Status {
        match self {
            FilesystemError::NotFound => base::error::Status::NotFound,
        }
    }
}

impl core::fmt::Display for FilesystemError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            FilesystemError::NotFound => write!(f, "File or directory not found"),
        }
    }
}
