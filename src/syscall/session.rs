use crate::{error, logln, process, session};

#[repr(C)]
struct CProcessInfo {
    num_threads: usize,
    time: usize,
    num_files: usize,
    num_directories: usize,
    working_directory: usize,
    working_directory_len: usize,
    name: usize,
    name_len: usize,
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
) -> isize {
    match code {
        GET_SESSION_ID_SYSCALL => process::get_current_thread()
            .process()
            .unwrap()
            .session_id()
            .unwrap(),
        GET_SESSION_PROCESSES_SYSCALL => match session::get_session_mut(arg1 as isize) {
            Some(session) => {
                let session = session.lock();
                let processes = session.get_processes();
                drop(session);

                (if arg2 != 0 && arg3 != 0 {
                    let buffer = match super::to_slice_mut(arg2, arg3) {
                        Ok(buffer) => buffer,
                        Err(error) => return error.to_return_code(),
                    };

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
                }) as isize
            }
            None => error::Status::InvalidSession.to_return_code(),
        },
        GET_PROCESS_INFO_SYSCALL => match session::get_session_mut(arg1 as isize) {
            Some(session) => {
                let output: *mut CProcessInfo = match super::to_ptr_mut(arg3) {
                    Ok(ptr) => ptr,
                    Err(error) => return error.to_return_code(),
                };

                let process = {
                    let session = session.lock();
                    match session.get_process(arg2 as isize) {
                        Some(process) => process,
                        None => return error::Status::NoProcess.to_return_code(),
                    }
                };

                let info = match process.get_process_info() {
                    Some(info) => info,
                    None => return error::Status::NoProcess.to_return_code(),
                };

                unsafe {
                    (*output).num_threads = info.num_threads;
                    (*output).time = info.time;
                    (*output).num_files = info.num_files;
                    (*output).num_directories = info.num_directories;

                    let output_working_directory = match super::to_slice_mut(
                        (*output).working_directory,
                        (*output).working_directory_len,
                    ) {
                        Ok(slice) => slice,
                        Err(error) => return error.to_return_code(),
                    };

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

                    let output_name = match super::to_slice_mut((*output).name, (*output).name_len)
                    {
                        Ok(slice) => slice,
                        Err(error) => return error.to_return_code(),
                    };

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

                0
            }
            None => error::Status::InvalidSession.to_return_code(),
        },
        _ => {
            logln!("Invalid session system call: {}", code);
            error::Status::InvalidRequestCode.to_return_code()
        }
    }
}
