use crate::SystemCallError;
use alloc::boxed::Box;
use base::{error::SYSTEM_CALLS_MODULE_NUMBER, log_error};
use ipc::{Pipe, PipeReader};
use process_types::ProcessTypes;

const CLOSE_PIPE_READ_SYSCALL: usize = 0xA000;
const CLOSE_PIPE_WRITE_SYSCALL: usize = 0xA001;
const CREATE_PIPE_SYSCALL: usize = 0xA002;
const READ_PIPE_SYSCALL: usize = 0xA003;
const WRITE_PIPE_SYSCALL: usize = 0xA004;

#[derive(Debug)]
enum PipeError {
    NotFound,
}

pub fn system_call(
    code: usize,
    arg1: usize,
    arg2: usize,
    arg3: usize,
    _arg4: usize,
    _arg5: usize,
) -> base::error::Result<isize> {
    match code {
        CLOSE_PIPE_READ_SYSCALL => {
            process::current_thread::<ProcessTypes>().lock(|thread| {
                thread
                    .process()
                    .lock(|process| process.descriptors_mut().remove_pipe_reader(arg1 as isize))
            });

            Ok(0)
        }
        CLOSE_PIPE_WRITE_SYSCALL => {
            process::current_thread::<ProcessTypes>().lock(|thread| {
                thread
                    .process()
                    .lock(|process| process.descriptors_mut().remove_pipe_writer(arg1 as isize))
            });

            Ok(0)
        }

        CREATE_PIPE_SYSCALL => {
            let ptr1 = super::to_ptr_mut(arg1)?;
            let ptr2 = super::to_ptr_mut(arg2)?;

            let (pipe_reader, pipe_writer) = Pipe::new::<ProcessTypes>();

            let (pipe_reader, pipe_writer) =
                process::current_thread::<ProcessTypes>().lock(|thread| {
                    thread.process().lock(|process| {
                        (
                            process.descriptors_mut().insert_pipe_reader(pipe_reader),
                            process.descriptors_mut().insert_pipe_writer(pipe_writer),
                        )
                    })
                });

            unsafe {
                *ptr1 = pipe_reader;
                *ptr2 = pipe_writer;
            }

            Ok(0)
        }
        READ_PIPE_SYSCALL => {
            let buffer = super::to_slice_mut(arg2, arg3)?;

            // Write upto buffer_len from buffer into pwd
            match process::current_thread::<ProcessTypes>().lock(|thread| {
                thread.process().lock(|process| {
                    process
                        .descriptors()
                        .pipe_reader(arg1 as isize)
                        .map(|pipe_reader| -> PipeReader<ProcessTypes> { pipe_reader.clone() })
                })
            }) {
                Some(pipe_reader) => {
                    pipe_reader.read(buffer);
                    Ok(buffer.len() as isize)
                }
                None => Err(Box::new(PipeError::NotFound)),
            }
        }

        WRITE_PIPE_SYSCALL => {
            let buffer = super::to_slice(arg2, arg3)?;

            // Write upto buffer_len from buffer into pwd
            match process::current_thread::<ProcessTypes>().lock(|thread| {
                thread.process().lock(|process| {
                    process
                        .descriptors()
                        .pipe_writer(arg1 as isize)
                        .map(|pipe_writer| pipe_writer.clone())
                })
            }) {
                Some(pipe_writer) => {
                    pipe_writer.write(buffer);
                    Ok(buffer.len() as isize)
                }
                None => Err(Box::new(PipeError::NotFound)),
            }
        }
        _ => {
            log_error!("Invalid pipe system call: {}", code);
            Err(Box::new(SystemCallError::InvalidCode))
        }
    }
}

impl base::error::Error for PipeError {
    fn module_number(&self) -> i32 {
        SYSTEM_CALLS_MODULE_NUMBER
    }

    fn error_number(&self) -> base::error::Status {
        match self {
            PipeError::NotFound => base::error::Status::NotFound,
        }
    }
}

impl core::fmt::Display for PipeError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            PipeError::NotFound => write!(f, "Pipe not found"),
        }
    }
}
