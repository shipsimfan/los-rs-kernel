use crate::SystemCallError;
use alloc::{borrow::ToOwned, boxed::Box, string::String};
use base::{error::SYSTEM_CALLS_MODULE_NUMBER, log_error, map::Mappable};
use filesystem::WorkingDirectory;
use process_types::ProcessTypes;

#[derive(Debug)]
enum SessionError {
    NotFound,
    ProcessNotFound,
}

#[repr(C)]
struct CProcessInfo {
    num_threads: usize,
    time: usize,
    num_files: usize,
    num_directories: usize,
    num_devices: usize,
    num_pipe_readers: usize,
    num_pipe_writers: usize,
    num_mutexes: usize,
    num_conditional_variables: usize,
    working_directory: usize,
    working_directory_len: usize,
    name: usize,
    name_len: usize,
}

struct ProcessInfo {
    num_threads: usize,
    time: usize,
    num_files: usize,
    num_directories: usize,
    num_devices: usize,
    num_pipe_readers: usize,
    num_pipe_writers: usize,
    num_mutexes: usize,
    num_conditional_variables: usize,
    working_directory: String,
    name: String,
}

const GET_SESSION_ID_SYSCALL: usize = 0x8000;
const GET_SESSION_PROCESSES_SYSCALL: usize = 0x8001;
const GET_PROCESS_INFO_SYSCALL: usize = 0x8002;

pub fn system_call(
    code: usize,
    arg1: usize,
    arg2: usize,
    arg3: usize,
    _arg4: usize,
    _arg5: usize,
) -> base::error::Result<isize> {
    match code {
        GET_SESSION_ID_SYSCALL => Ok(process::current_thread::<ProcessTypes>()
            .lock(|thread| thread.process().lock(|process| process.owner().id()))),
        GET_SESSION_PROCESSES_SYSCALL => {
            let processes = match sessions::get_session::<ProcessTypes>(arg1 as isize) {
                Some(session) => session.lock(|session| session.processes()),
                None => return Err(Box::new(SessionError::NotFound)),
            };

            Ok((if arg2 != 0 && arg3 != 0 {
                let buffer = super::to_slice_mut(arg2, arg3)?;
                let mut i = 0;
                for process in processes {
                    if i >= arg3 {
                        break;
                    }

                    buffer[i] = process;
                    i += 1;
                }

                i
            } else {
                processes.len()
            }) as isize)
        }
        GET_PROCESS_INFO_SYSCALL => {
            let output: *mut CProcessInfo = super::to_ptr_mut(arg3)?;
            let info = match sessions::get_session::<ProcessTypes>(arg1 as isize) {
                Some(session) => match session.lock(|session| {
                    session.get_process(arg2 as isize).map(|process| {
                        process.lock(|process| ProcessInfo {
                            num_threads: process.thread_count(),
                            time: process.time() as usize,
                            num_files: process.descriptors().file_count(),
                            num_directories: process.descriptors().directory_count(),
                            num_devices: process.descriptors().device_count(),
                            num_pipe_readers: process.descriptors().pipe_reader_count(),
                            num_pipe_writers: process.descriptors().pipe_writer_count(),
                            num_mutexes: process.descriptors().conditional_variable_count(),
                            num_conditional_variables: process.descriptors().mutex_count(),
                            working_directory: match process.descriptors().working_directory() {
                                Some(directory) => directory.get_full_path(),
                                None => String::new(),
                            },
                            name: process.name().to_owned(),
                        })
                    })
                }) {
                    Some(info) => match info {
                        Some(info) => info,
                        None => return Err(Box::new(SessionError::ProcessNotFound)),
                    },
                    None => return Err(Box::new(SessionError::ProcessNotFound)),
                },
                None => return Err(Box::new(SessionError::NotFound)),
            };

            unsafe {
                (*output).num_threads = info.num_threads;
                (*output).time = info.time;
                (*output).num_files = info.num_files;
                (*output).num_directories = info.num_directories;
                (*output).num_devices = info.num_devices;
                (*output).num_pipe_readers = info.num_pipe_readers;
                (*output).num_pipe_writers = info.num_pipe_writers;
                (*output).num_mutexes = info.num_mutexes;
                (*output).num_conditional_variables = info.num_conditional_variables;

                let output_working_directory = super::to_slice_mut(
                    (*output).working_directory,
                    (*output).working_directory_len,
                )?;

                let mut i = 0;
                let mut working_directory = info.working_directory.bytes();
                while i < output_working_directory.len() - 1 {
                    match working_directory.next() {
                        Some(b) => output_working_directory[i] = b,
                        None => break,
                    }

                    i += 1;
                }

                output_working_directory[i] = 0;

                let output_name = super::to_slice_mut((*output).name, (*output).name_len)?;
                i = 0;
                let mut name = info.name.bytes();
                while i < output_name.len() - 1 {
                    match name.next() {
                        Some(b) => output_name[i] = b,
                        None => break,
                    }

                    i += 1;
                }

                output_name[i] = 0;
            }

            Ok(0)
        }
        _ => {
            log_error!("Invalid session system call: {}", code);
            Err(Box::new(SystemCallError::InvalidCode))
        }
    }
}

impl base::error::Error for SessionError {
    fn module_number(&self) -> i32 {
        SYSTEM_CALLS_MODULE_NUMBER
    }

    fn error_number(&self) -> base::error::Status {
        match self {
            SessionError::NotFound | SessionError::ProcessNotFound => base::error::Status::NotFound,
        }
    }
}

impl core::fmt::Display for SessionError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            SessionError::NotFound => write!(f, "Session not found"),
            SessionError::ProcessNotFound => write!(f, "Process not found"),
        }
    }
}
