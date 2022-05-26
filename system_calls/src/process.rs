use crate::SystemCallError;
use alloc::{boxed::Box, format, string::ToString, vec::Vec};
use base::{
    error::SYSTEM_CALLS_MODULE_NUMBER,
    log_info,
    map::{Mappable, INVALID_ID},
};
use filesystem::WorkingDirectory;
use process_types::ProcessTypes;
use program_loader::CStandardIO;

const WAIT_PROCESS_SYSCALL: usize = 0x0000;
const EXECUTE_SYSCALL: usize = 0x0001;
const GET_CURRENT_WORKING_DIRECTORY: usize = 0x0002;
const SET_CURRENT_WORKING_DIRECTORY: usize = 0x0003;
const EXIT_PROCESS_SYSCALL: usize = 0x0004;
const KILL_PROCESS_SYSCALL: usize = 0x0005;
const KILL_THREAD_SYSCALL: usize = 0x0006;

#[derive(Debug)]
enum ProcessError {
    NotFound,
}

pub fn system_call(
    code: usize,
    arg1: usize,
    arg2: usize,
    arg3: usize,
    arg4: usize,
    arg5: usize,
) -> base::error::Result<isize> {
    match code {
        WAIT_PROCESS_SYSCALL => {
            match process::wait_process::<ProcessTypes>(
                &match process::current_thread::<ProcessTypes>().lock(|thread| {
                    thread.process().lock(|process| {
                        process.owner().lock(|session| {
                            session
                                .get_process(arg1 as isize)
                                .map(|process| process.clone())
                        })
                    })
                }) {
                    Some(process) => process,
                    None => return Err(Box::new(ProcessError::NotFound)),
                },
            ) {
                Some(status) => Ok(status),
                None => Err(Box::new(ProcessError::NotFound)),
            }
        }
        EXECUTE_SYSCALL => {
            let filepath = super::to_str(arg1)?;
            let argv = super::to_slice_null(arg2)?;
            let envp = super::to_slice_null(arg3)?;

            let mut args = Vec::with_capacity(argv.len());
            for arg in argv {
                let arg = super::to_str(*arg)?;
                args.push(arg.to_string());
            }

            let mut environment = Vec::with_capacity(envp.len());
            for env in envp {
                let env = super::to_str(*env)?;
                environment.push(env.to_string());
            }

            let stdio = unsafe { (*super::to_ptr_mut::<CStandardIO>(arg4)?).to_stdio()? };

            let inherit_signals = arg5 != 0;

            program_loader::execute(
                filepath,
                args.as_slice(),
                environment.as_slice(),
                stdio,
                inherit_signals,
            )
            .map(|process| process.lock(|process| process.id()).unwrap_or(INVALID_ID))
        }
        GET_CURRENT_WORKING_DIRECTORY => {
            if arg1 >= memory::KERNEL_VMA || arg1 + arg2 >= memory::KERNEL_VMA {
                Err(Box::new(SystemCallError::ArgumentSecurity))
            } else {
                let mut path = match process::current_thread::<ProcessTypes>().lock(|thread| {
                    thread.process().lock(|process| {
                        process
                            .descriptors()
                            .working_directory()
                            .map(|directory| directory.directory().clone())
                    })
                }) {
                    Some(directory) => directory.lock(|directory| directory.construct_path_name()),
                    None => format!("?"),
                };

                path.push(0 as char);

                let copy_len = if path.len() < arg2 { path.len() } else { arg2 };

                unsafe { core::ptr::copy_nonoverlapping(path.as_ptr(), arg1 as *mut u8, copy_len) };

                Ok((copy_len & 0x7FFFFFFFFFFF) as isize)
            }
        }
        SET_CURRENT_WORKING_DIRECTORY => {
            let path = super::to_str(arg1)?;

            let directory = filesystem::open_directory::<ProcessTypes>(path, None)?;
            process::current_thread::<ProcessTypes>().lock(|thread| {
                thread
                    .process()
                    .lock(|process| process.descriptors_mut().set_working_directory(directory))
            });
            Ok(0)
        }
        EXIT_PROCESS_SYSCALL => process::exit_process::<ProcessTypes>(arg1 as isize),
        KILL_PROCESS_SYSCALL => {
            process::kill_process::<ProcessTypes>(
                &match process::current_thread::<ProcessTypes>().lock(|thread| {
                    thread.process().lock(|process| {
                        process.owner().lock(|session| {
                            session
                                .get_process(arg1 as isize)
                                .map(|process| process.clone())
                        })
                    })
                }) {
                    Some(process) => process,
                    None => return Err(Box::new(ProcessError::NotFound)),
                },
                128,
            );
            Ok(0)
        }
        KILL_THREAD_SYSCALL => {
            process::kill_thread(
                &match process::current_thread::<ProcessTypes>().lock(|thread| {
                    thread.process().lock(|process| {
                        process
                            .get_thread(arg1 as isize)
                            .map(|thread| thread.clone())
                    })
                }) {
                    Some(thread) => thread,
                    None => return Err(Box::new(ProcessError::NotFound)),
                },
                128,
            );
            Ok(0)
        }
        _ => {
            log_info!("Invalid process system call: {}", code);
            Err(Box::new(SystemCallError::InvalidCode))
        }
    }
}

impl base::error::Error for ProcessError {
    fn module_number(&self) -> i32 {
        SYSTEM_CALLS_MODULE_NUMBER
    }

    fn error_number(&self) -> base::error::Status {
        match self {
            ProcessError::NotFound => base::error::Status::NotFound,
        }
    }
}

impl core::fmt::Display for ProcessError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            ProcessError::NotFound => write!(f, "Process not found"),
        }
    }
}
