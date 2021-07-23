use crate::{error, logln, memory::KERNEL_VMA, process};

const EXIT_THREAD_SYSCALL: usize = 0x1000;
const WAIT_THREAD_SYSCALL: usize = 0x1001;
const CREATE_THREAD_SYSCALL: usize = 0x1002;

pub fn system_call(
    code: usize,
    arg1: usize,
    arg2: usize,
    _arg3: usize,
    _arg4: usize,
    _arg5: usize,
) -> isize {
    match code {
        EXIT_THREAD_SYSCALL => process::exit_thread((arg1 & 0x7FFFFFFFFFFF) as isize),
        WAIT_THREAD_SYSCALL => process::wait_thread((arg1 & 0x7FFFFFFFFFFF) as isize),
        CREATE_THREAD_SYSCALL => {
            if arg1 >= KERNEL_VMA {
                error::Status::InvalidArgument as isize
            } else {
                process::create_thread_raw(arg1, arg2)
            }
        }
        _ => {
            logln!("Invalid thread system call: {}", code);
            error::Status::InvalidSystemCall as isize
        }
    }
}
