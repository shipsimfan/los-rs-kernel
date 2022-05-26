use crate::SystemCallError;
use alloc::boxed::Box;
use base::{
    error::SYSTEM_CALLS_MODULE_NUMBER,
    log_error,
    map::{Mappable, INVALID_ID},
};
use process_types::ProcessTypes;

#[derive(Debug)]
enum ThreadError {
    NotFound,
}

const EXIT_THREAD_SYSCALL: usize = 0x1000;
const WAIT_THREAD_SYSCALL: usize = 0x1001;
const CREATE_THREAD_SYSCALL: usize = 0x1002;
const SET_TLS_BASE_SYSCALL: usize = 0x1003;

pub fn system_call(
    code: usize,
    arg1: usize,
    arg2: usize,
    _arg3: usize,
    _arg4: usize,
    _arg5: usize,
) -> base::error::Result<isize> {
    match code {
        EXIT_THREAD_SYSCALL => process::exit_thread::<ProcessTypes>(arg1 as isize),
        WAIT_THREAD_SYSCALL => {
            match process::wait_thread(&match process::current_thread::<ProcessTypes>().lock(
                |thread| {
                    thread.process().lock(|process| {
                        process
                            .get_thread(arg1 as isize)
                            .map(|thread| thread.clone())
                    })
                },
            ) {
                Some(thread) => thread,
                None => return Err(Box::new(ThreadError::NotFound)),
            }) {
                Some(result) => Ok(result),
                None => Err(Box::new(ThreadError::NotFound)),
            }
        }
        CREATE_THREAD_SYSCALL => {
            if arg1 >= memory::KERNEL_VMA {
                Err(Box::new(SystemCallError::ArgumentSecurity))
            } else {
                Ok(process::create_thread_usize::<ProcessTypes>(arg1, arg2)
                    .lock(|thread| thread.id())
                    .unwrap_or(INVALID_ID))
            }
        }
        SET_TLS_BASE_SYSCALL => {
            if arg1 >= memory::KERNEL_VMA {
                Err(Box::new(SystemCallError::ArgumentSecurity))
            } else {
                process::current_thread::<ProcessTypes>()
                    .lock(|thread| thread.set_tls_base(arg1 as usize));
                Ok(0)
            }
        }
        _ => {
            log_error!("Invalid thread system call: {}", code);
            Err(Box::new(SystemCallError::InvalidCode))
        }
    }
}

impl base::error::Error for ThreadError {
    fn module_number(&self) -> i32 {
        SYSTEM_CALLS_MODULE_NUMBER
    }

    fn error_number(&self) -> base::error::Status {
        match self {
            ThreadError::NotFound => base::error::Status::NotFound,
        }
    }
}

impl core::fmt::Display for ThreadError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            ThreadError::NotFound => write!(f, "Thread not found"),
        }
    }
}
