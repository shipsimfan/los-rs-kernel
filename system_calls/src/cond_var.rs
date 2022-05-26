use crate::SystemCallError;
use alloc::boxed::Box;
use base::{error::SYSTEM_CALLS_MODULE_NUMBER, log_error};
use ipc::ConditionalVariable;
use process_types::ProcessTypes;

#[derive(Debug)]
enum ConditionalVariableError {
    NotFound,
}

const CREATE_COND_VAR_SYSCALL: usize = 0xC000;
const WAIT_COND_VAR_SYSCALL: usize = 0xC001;
const SIGNAL_COND_VAR: usize = 0xC002;
const BROADCAST_COND_VAR: usize = 0xC003;
const DESTROY_COND_VAR_SYSCALL: usize = 0xC004;

pub fn system_call(
    code: usize,
    arg1: usize,
    _arg2: usize,
    _arg3: usize,
    _arg4: usize,
    _arg5: usize,
) -> base::error::Result<isize> {
    match code {
        CREATE_COND_VAR_SYSCALL => {
            let conditional_variable = ConditionalVariable::<ProcessTypes>::new();

            Ok(process::current_thread::<ProcessTypes>().lock(|thread| {
                thread.process().lock(|process| {
                    process
                        .descriptors_mut()
                        .insert_conditional_variable(conditional_variable)
                })
            }))
        }
        DESTROY_COND_VAR_SYSCALL => {
            process::current_thread::<ProcessTypes>().lock(|thread| {
                thread.process().lock(|process| {
                    process
                        .descriptors_mut()
                        .remove_conditional_variable(arg1 as isize)
                })
            });

            Ok(0)
        }
        WAIT_COND_VAR_SYSCALL => {
            match process::current_thread::<ProcessTypes>().lock(|thread| {
                thread.process().lock(|process| {
                    process
                        .descriptors()
                        .conditional_variable(arg1 as isize)
                        .map(|conditional_variable| conditional_variable.clone())
                })
            }) {
                Some(conditional_variable) => conditional_variable.wait(),
                None => return Err(Box::new(ConditionalVariableError::NotFound)),
            };
            Ok(0)
        }
        SIGNAL_COND_VAR => {
            match process::current_thread::<ProcessTypes>().lock(|thread| {
                thread.process().lock(|process| {
                    process
                        .descriptors()
                        .conditional_variable(arg1 as isize)
                        .map(|conditional_variable| conditional_variable.clone())
                })
            }) {
                Some(conditional_variable) => conditional_variable.signal(),
                None => return Err(Box::new(ConditionalVariableError::NotFound)),
            };
            Ok(0)
        }
        BROADCAST_COND_VAR => {
            match process::current_thread::<ProcessTypes>().lock(|thread| {
                thread.process().lock(|process| {
                    process
                        .descriptors()
                        .conditional_variable(arg1 as isize)
                        .map(|conditional_variable| conditional_variable.clone())
                })
            }) {
                Some(conditional_variable) => conditional_variable.broadcast(),
                None => return Err(Box::new(ConditionalVariableError::NotFound)),
            };
            Ok(0)
        }
        _ => {
            log_error!(
                "Invalid userspace conditional variablt system call: {}",
                code
            );
            Err(Box::new(SystemCallError::InvalidCode))
        }
    }
}

impl base::error::Error for ConditionalVariableError {
    fn module_number(&self) -> i32 {
        SYSTEM_CALLS_MODULE_NUMBER
    }

    fn error_number(&self) -> base::error::Status {
        match self {
            ConditionalVariableError::NotFound => base::error::Status::NotFound,
        }
    }
}

impl core::fmt::Display for ConditionalVariableError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            ConditionalVariableError::NotFound => write!(f, "Conditional variable not found"),
        }
    }
}
