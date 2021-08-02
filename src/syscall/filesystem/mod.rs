use crate::{
    error,
    filesystem::{self, SeekFrom},
    logln, process,
};

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

pub fn system_call(
    code: usize,
    arg1: usize,
    arg2: usize,
    arg3: usize,
    _arg4: usize,
    _arg5: usize,
) -> isize {
    match code {
        OPEN_FILE_SYSCALL => {
            let filepath = match super::to_str(arg1) {
                Ok(str) => str,
                Err(status) => return status.to_return_code(),
            };
            match process::get_current_thread_mut()
                .get_process_mut()
                .open_file(filepath, arg2)
            {
                Ok(fd) => fd,
                Err(status) => status.to_return_code(),
            }
        }
        CLOSE_FILE_SYSCALL => {
            process::get_current_thread_mut()
                .get_process_mut()
                .close_file((arg1 & 0x7FFFFFFFFFFF) as isize);
            0
        }
        SEEK_FILE_SYSCALL => {
            let file = match process::get_current_thread_mut()
                .get_process_mut()
                .get_file((arg1 & 0x7FFFFFFFFFFF) as isize)
            {
                Ok(file) => file,
                Err(status) => return status.to_return_code(),
            };

            (file.seek(arg2, SeekFrom::from(arg3)) & 0x7FFFFFFFFFFF) as isize
        }
        READ_FILE_SYSCALL => {
            let file = match process::get_current_thread_mut()
                .get_process_mut()
                .get_file((arg1 & 0x7FFFFFFFFFFF) as isize)
            {
                Ok(file) => file,
                Err(status) => return status.to_return_code(),
            };

            let buffer = match super::to_slice_mut(arg2, arg3) {
                Ok(slice) => slice,
                Err(status) => return status.to_return_code(),
            };

            match file.read(buffer) {
                Ok(bytes_read) => bytes_read,
                Err(status) => status.to_return_code(),
            }
        }
        OPEN_DIRECTORY_SYSCALL => {
            let path = match super::to_str(arg1) {
                Ok(str) => str,
                Err(status) => return status.to_return_code(),
            };
            match process::get_current_thread_mut()
                .get_process_mut()
                .open_directory(path)
            {
                Ok(dd) => dd,
                Err(status) => status.to_return_code(),
            }
        }
        CLOSE_DIRECTORY_SYSCALL => {
            process::get_current_thread_mut()
                .get_process_mut()
                .close_directory((arg1 & 0x7FFFFFFFFFFF) as isize);
            0
        }
        READ_DIRECTORY_SYSCALL => {
            let desintation = match super::to_ptr_mut(arg2) {
                Ok(ptr) => ptr,
                Err(status) => return status.to_return_code(),
            };

            match process::get_current_thread_mut()
                .get_process_mut()
                .read_directory((arg1 & 0x7FFFFFFFFFFF) as isize)
            {
                Ok(directory_entry) => match directory_entry {
                    Some(dirent) => {
                        unsafe { *desintation = dirent };
                        1
                    }
                    None => 0,
                },
                Err(status) => status.to_return_code(),
            }
        }
        TRUNCATE_FILE_SYSCALL => {
            match process::get_current_thread_mut()
                .get_process_mut()
                .get_file((arg1 & 0x7FFFFFFFFFFF) as isize)
            {
                Ok(file_descriptor) => match file_descriptor.set_length(arg2) {
                    Ok(()) => 0,
                    Err(status) => status.to_return_code(),
                },
                Err(status) => status.to_return_code(),
            }
        }
        WRITE_FILE_SYSCALL => {
            let file = match process::get_current_thread_mut()
                .get_process_mut()
                .get_file((arg1 & 0x7FFFFFFFFFFF) as isize)
            {
                Ok(file) => file,
                Err(status) => return status.to_return_code(),
            };

            let buffer = match super::to_slice_mut(arg2, arg3) {
                Ok(slice) => slice,
                Err(status) => return status.to_return_code(),
            };

            match file.write(buffer) {
                Ok(bytes_read) => bytes_read,
                Err(status) => status.to_return_code(),
            }
        }
        REMOVE_FILE_SYSCALL => {
            let path = match super::to_str(arg1) {
                Ok(str) => str,
                Err(status) => return status.to_return_code(),
            };

            match filesystem::remove_file(path) {
                Ok(()) => 0,
                Err(status) => status.to_return_code(),
            }
        }
        REMOVE_DIRECTORY_SYSCALL => {
            let path = match super::to_str(arg1) {
                Ok(str) => str,
                Err(status) => return status.to_return_code(),
            };

            match filesystem::remove_directory(path) {
                Ok(()) => 0,
                Err(status) => status.to_return_code(),
            }
        }
        _ => {
            logln!("Invalid filesystem system call: {}", code);
            error::Status::InvalidRequestCode.to_return_code()
        }
    }
}
