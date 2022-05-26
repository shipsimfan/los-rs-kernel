use crate::SystemCallError;
use alloc::boxed::Box;
use base::{error::SYSTEM_CALLS_MODULE_NUMBER, log_error};
use ipc::SignalHandler;
use process_types::ProcessTypes;

#[derive(Debug)]
enum SignalError {
    NotFound,
    ProcessNotFound,
}

const RAISE_SESSION_SYSCALL: usize = 0x9000;
const RAISE_PROCESS_SYSCALL: usize = 0x9001;
const RAISE_SELF_SYSCALL: usize = 0x9002;
const SET_SIGNAL_TYPE_SYSCALL: usize = 0x9003;
const MASK_SIGNAL_SYSCALL: usize = 0x9004;
const UNMASK_SIGNAL_SYSCALL: usize = 0x9005;
const SET_USERSPACE_SIGNAL_HANDLER_SYSCALL: usize = 0x9006;

pub fn system_call(
    code: usize,
    arg1: usize,
    arg2: usize,
    arg3: usize,
    _arg4: usize,
    _arg5: usize,
) -> base::error::Result<isize> {
    match code {
        RAISE_SESSION_SYSCALL => match sessions::get_session::<ProcessTypes>(arg1 as isize) {
            Some(session) => session.lock(|session| -> base::error::Result<isize> {
                match session.get_process(arg2 as isize) {
                    Some(process) => process
                        .lock(|process| {
                            process.signals_mut().raise((arg3 & 0xFF) as u8);
                            Ok(0)
                        })
                        .unwrap_or(Err(Box::new(SignalError::ProcessNotFound))),
                    None => Err(Box::new(SignalError::ProcessNotFound)),
                }
            }),
            None => Err(Box::new(SignalError::NotFound)),
        },
        RAISE_PROCESS_SYSCALL => {
            let session = process::current_thread::<ProcessTypes>()
                .lock(|thread| thread.process().lock(|process| process.owner().clone()));

            session.lock(|session| -> base::error::Result<isize> {
                match session.get_process(arg1 as isize) {
                    Some(process) => {
                        process.lock(|process| process.signals_mut().raise((arg2 & 0xFF) as u8));
                        Ok(0)
                    }
                    None => Err(Box::new(SignalError::ProcessNotFound)),
                }
            })
        }
        RAISE_SELF_SYSCALL => {
            process::current_thread::<ProcessTypes>().lock(|thread| {
                thread
                    .process()
                    .lock(|process| process.signals_mut().raise((arg1 & 0xFF) as u8))
            });
            Ok(0)
        }
        SET_SIGNAL_TYPE_SYSCALL => {
            let handler = match SignalHandler::from(arg2) {
                Some(handler) => handler,
                None => return Ok(0),
            };

            process::current_thread::<ProcessTypes>().lock(|thread| {
                thread.process().lock(|process| {
                    process
                        .signals_mut()
                        .set_handler((arg1 & 0xFF) as u8, handler)
                })
            });
            Ok(0)
        }
        MASK_SIGNAL_SYSCALL => {
            process::current_thread::<ProcessTypes>().lock(|thread| {
                thread
                    .process()
                    .lock(|process| process.signals_mut().mask((arg1 & 0xFF) as u8, true))
            });
            Ok(0)
        }
        UNMASK_SIGNAL_SYSCALL => {
            process::current_thread::<ProcessTypes>().lock(|thread| {
                thread
                    .process()
                    .lock(|process| process.signals_mut().mask((arg1 & 0xFF) as u8, false))
            });
            Ok(0)
        }
        SET_USERSPACE_SIGNAL_HANDLER_SYSCALL => {
            process::current_thread::<ProcessTypes>().lock(|thread| {
                thread
                    .process()
                    .lock(|process| process.signals_mut().set_userspace_handler(arg1))
            });
            Ok(0)
        }
        _ => {
            log_error!("Invalid signal system call: {}", code);
            Err(Box::new(SystemCallError::InvalidCode))
        }
    }
}

impl base::error::Error for SignalError {
    fn module_number(&self) -> i32 {
        SYSTEM_CALLS_MODULE_NUMBER
    }

    fn error_number(&self) -> base::error::Status {
        match self {
            SignalError::NotFound | SignalError::ProcessNotFound => base::error::Status::NotFound,
        }
    }
}

impl core::fmt::Display for SignalError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            SignalError::NotFound => write!(f, "Session not found"),
            SignalError::ProcessNotFound => write!(f, "Process not found"),
        }
    }
}
