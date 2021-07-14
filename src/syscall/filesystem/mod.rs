use crate::{filesystem::SeekFrom, logln, memory::KERNEL_VMA, process};

const OPEN_FILE_SYSCALL: usize = 0x2000;
const CLOSE_FILE_SYSCALL: usize = 0x2001;
const SEEK_FILE_SYSCALL: usize = 0x2002;
const READ_FILE_SYSCALL: usize = 0x2003;

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
            if arg1 >= KERNEL_VMA || arg1 + arg2 >= KERNEL_VMA {
                usize::MAX
            } else {
                let slice = unsafe { core::slice::from_raw_parts(arg1 as *const u8, arg2) };
                let filepath = match core::str::from_utf8(slice) {
                    Ok(str) => str,
                    Err(_) => return usize::MAX,
                };
                match process::get_current_thread_mut()
                    .get_process_mut()
                    .open_file(filepath)
                {
                    Ok(bytes_read) => bytes_read,
                    Err(_) => usize::MAX,
                }
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
            if arg2 >= KERNEL_VMA || arg2 + arg3 >= KERNEL_VMA {
                usize::MAX
            } else {
                let file = match process::get_current_thread_mut()
                    .get_process_mut()
                    .get_file(arg1)
                {
                    Ok(file) => file,
                    Err(_) => return usize::MAX,
                };

                let buffer = unsafe { core::slice::from_raw_parts_mut(arg2 as *mut u8, arg3) };

                match file.read(buffer) {
                    Ok(bytes_read) => bytes_read,
                    Err(_) => usize::MAX,
                }
            }
        }
        _ => {
            logln!("Invalid filesystem system call: {}", code);
            usize::MAX
        }
    }
}
