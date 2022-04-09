use crate::{
    error, filesystem, logln,
    memory::KERNEL_VMA,
    process::{self, CStandardIO},
};
use alloc::{string::ToString, vec::Vec};

const WAIT_PROCESS_SYSCALL: usize = 0x0000;
const EXECUTE_SYSCALL: usize = 0x0001;
const GET_CURRENT_WORKING_DIRECTORY: usize = 0x0002;
const SET_CURRENT_WORKING_DIRECTORY: usize = 0x0003;
const EXIT_PROCESS_SYSCALL: usize = 0x0004;
const KILL_PROCESS_SYSCALL: usize = 0x0005;
const KILL_THREAD_SYSCALL: usize = 0x0006;

pub fn system_call(
    code: usize,
    arg1: usize,
    arg2: usize,
    arg3: usize,
    arg4: usize,
    arg5: usize,
) -> isize {
    match code {
        WAIT_PROCESS_SYSCALL => process::wait_process((arg1 & 0x7FFFFFFFFFFF) as isize),
        EXECUTE_SYSCALL => {
            let filepath = match super::to_str(arg1) {
                Ok(str) => str,
                Err(status) => return status.to_return_code(),
            };

            let argv = match super::to_slice_null(arg2) {
                Ok(argv) => argv,
                Err(status) => return status.to_return_code(),
            };

            let envp = match super::to_slice_null(arg3) {
                Ok(envp) => envp,
                Err(status) => return status.to_return_code(),
            };

            let mut args = Vec::with_capacity(argv.len());
            for arg in argv {
                let arg = match super::to_str(*arg) {
                    Ok(arg) => arg,
                    Err(status) => return status.to_return_code(),
                };

                args.push(arg.to_string());
            }

            let mut environment = Vec::with_capacity(envp.len());
            for env in envp {
                let env = match super::to_str(*env) {
                    Ok(env) => env,
                    Err(status) => return status.to_return_code(),
                };

                environment.push(env.to_string());
            }

            let stdio = match super::to_ptr_mut::<CStandardIO>(arg4) {
                Ok(stdio_ptr) => match unsafe { (*stdio_ptr).to_stdio() } {
                    Ok(stdio) => stdio,
                    Err(status) => return status.to_return_code(),
                },
                Err(status) => return status.to_return_code(),
            };

            let inherit_signals = arg5 != 0;

            match process::execute(filepath, args, environment, stdio, inherit_signals) {
                Ok(pid) => pid,
                Err(status) => status.to_return_code(),
            }
        }
        GET_CURRENT_WORKING_DIRECTORY => {
            if arg1 >= KERNEL_VMA || arg1 + arg2 >= KERNEL_VMA {
                error::Status::ArgumentSecurity.to_return_code()
            } else {
                let process_lock = process::get_current_thread()
                    .process()
                    .unwrap()
                    .upgrade()
                    .unwrap();
                let mut process = process_lock.lock();
                let mut path = match process.current_working_directory() {
                    Some(dir) => dir.get_full_path(),
                    None => return error::Status::NotSupported.to_return_code(),
                };

                path.push(0 as char);

                let copy_len = if path.len() < arg2 { path.len() } else { arg2 };

                unsafe { core::ptr::copy_nonoverlapping(path.as_ptr(), arg1 as *mut u8, copy_len) };

                (copy_len & 0x7FFFFFFFFFFF) as isize
            }
        }
        SET_CURRENT_WORKING_DIRECTORY => {
            let path = match super::to_str(arg1) {
                Ok(str) => str,
                Err(status) => return status.to_return_code(),
            };

            match filesystem::open_directory(path, None) {
                Ok(directory) => {
                    process::get_current_thread()
                        .process()
                        .unwrap()
                        .set_current_working_directory(directory);
                    0
                }
                Err(status) => status.to_return_code(),
            }
        }
        EXIT_PROCESS_SYSCALL => process::exit_process((arg1 & 0x7FFFFFFFFFFF) as isize),
        KILL_PROCESS_SYSCALL => {
            process::kill_process((arg1 & 0x7FFFFFFFFFFF) as isize);
            0
        }
        KILL_THREAD_SYSCALL => {
            process::kill_thread((arg1 & 0x7FFFFFFFFFFF) as isize);
            0
        }
        _ => {
            logln!("Invalid process system call: {}", code);
            error::Status::InvalidRequestCode.to_return_code()
        }
    }
}
