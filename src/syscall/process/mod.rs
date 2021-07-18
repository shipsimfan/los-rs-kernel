use crate::{filesystem, logln, memory::KERNEL_VMA, process};

const WAIT_PROCESS_SYSCALL: usize = 0x0000;
const EXECUTE_SYSCALL: usize = 0x0001;
const GET_CURRENT_WORKING_DIRECTORY: usize = 0x0002;
const SET_CURRENT_WORKING_DIRECTORY: usize = 0x0003;

pub fn system_call(
    code: usize,
    arg1: usize,
    arg2: usize,
    _arg3: usize,
    _arg4: usize,
    _arg5: usize,
) -> usize {
    match code {
        WAIT_PROCESS_SYSCALL => process::wait_process(arg1),
        EXECUTE_SYSCALL => {
            let filepath = match super::to_str(arg1) {
                Ok(str) => str,
                Err(_) => return usize::MAX,
            };

            match process::execute(filepath) {
                Ok(pid) => pid,
                Err(_) => usize::MAX,
            }
        }
        GET_CURRENT_WORKING_DIRECTORY => {
            if arg1 >= KERNEL_VMA || arg1 + arg2 >= KERNEL_VMA {
                usize::MAX
            } else {
                let mut path = match process::get_current_thread_mut()
                    .get_process_mut()
                    .get_current_working_directory()
                {
                    Some(dir) => dir.get_full_path(),
                    None => return usize::MAX,
                };

                path.push(0 as char);

                let copy_len = if path.len() < arg2 { path.len() } else { arg2 };

                unsafe { core::ptr::copy_nonoverlapping(path.as_ptr(), arg1 as *mut u8, copy_len) };

                copy_len
            }
        }
        SET_CURRENT_WORKING_DIRECTORY => {
            let path = match super::to_str(arg1) {
                Ok(str) => str,
                Err(_) => return usize::MAX,
            };

            match filesystem::open_directory(path) {
                Ok(directory) => {
                    process::get_current_thread_mut()
                        .get_process_mut()
                        .set_current_working_directory(directory);
                    0
                }
                Err(_) => usize::MAX,
            }
        }
        _ => {
            logln!("Invalid process system call: {}", code);
            usize::MAX
        }
    }
}
