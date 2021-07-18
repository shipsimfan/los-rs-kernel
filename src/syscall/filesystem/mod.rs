use crate::{filesystem::SeekFrom, logln, process};

const OPEN_FILE_SYSCALL: usize = 0x2000;
const CLOSE_FILE_SYSCALL: usize = 0x2001;
const SEEK_FILE_SYSCALL: usize = 0x2002;
const READ_FILE_SYSCALL: usize = 0x2003;
const OPEN_DIRECTORY_SYSCALL: usize = 0x2004;
const CLOSE_DIRECTORY_SYSCALL: usize = 0x2005;
const READ_DIRECTORY_SYSCALL: usize = 0x2006;

pub fn system_call(
    code: usize,
    arg1: usize,
    arg2: usize,
    arg3: usize,
    _arg4: usize,
    _arg5: usize,
) -> usize {
    match code {
        OPEN_FILE_SYSCALL => {
            let filepath = match super::to_str(arg1) {
                Ok(str) => str,
                Err(_) => return usize::MAX,
            };
            match process::get_current_thread_mut()
                .get_process_mut()
                .open_file(filepath)
            {
                Ok(fd) => fd,
                Err(_) => usize::MAX,
            }
        }
        CLOSE_FILE_SYSCALL => {
            process::get_current_thread_mut()
                .get_process_mut()
                .close_file(arg1);
            0
        }
        SEEK_FILE_SYSCALL => {
            let file = match process::get_current_thread_mut()
                .get_process_mut()
                .get_file(arg1)
            {
                Ok(file) => file,
                Err(_) => return usize::MAX,
            };

            file.seek(arg2, SeekFrom::from(arg3))
        }
        READ_FILE_SYSCALL => {
            let file = match process::get_current_thread_mut()
                .get_process_mut()
                .get_file(arg1)
            {
                Ok(file) => file,
                Err(_) => return usize::MAX,
            };

            let buffer = match super::to_slice_mut(arg2, arg3) {
                Ok(slice) => slice,
                Err(_) => return usize::MAX,
            };

            match file.read(buffer) {
                Ok(bytes_read) => bytes_read,
                Err(_) => usize::MAX,
            }
        }
        OPEN_DIRECTORY_SYSCALL => {
            let path = match super::to_str(arg1) {
                Ok(str) => str,
                Err(_) => return usize::MAX,
            };
            match process::get_current_thread_mut()
                .get_process_mut()
                .open_directory(path)
            {
                Ok(dd) => dd,
                Err(_) => usize::MAX,
            }
        }
        CLOSE_DIRECTORY_SYSCALL => {
            process::get_current_thread_mut()
                .get_process_mut()
                .close_directory(arg1);
            0
        }
        READ_DIRECTORY_SYSCALL => {
            let desintation = match super::to_ptr_mut(arg2) {
                Ok(ptr) => ptr,
                Err(_) => return usize::MAX,
            };

            match process::get_current_thread_mut()
                .get_process_mut()
                .read_directory(arg1)
            {
                Ok(directory_entry) => match directory_entry {
                    Some(dirent) => {
                        unsafe { *desintation = dirent };
                        1
                    }
                    None => 0,
                },
                Err(_) => usize::MAX,
            }
        }
        _ => {
            logln!("Invalid filesystem system call: {}", code);
            usize::MAX
        }
    }
}
