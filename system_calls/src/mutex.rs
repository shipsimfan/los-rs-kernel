use crate::SystemCallError;
use alloc::boxed::Box;
use base::{error::SYSTEM_CALLS_MODULE_NUMBER, log_error};
use ipc::Mutex;
use process_types::ProcessTypes;

const CREATE_MUTEX_SYSCALL: usize = 0xB000;
const LOCK_MUTEX_SYSCALL: usize = 0xB001;
const TRY_LOCK_MUTEX_SYSCALL: usize = 0xB002;
const UNLOCK_MUTEX_SYSCALL: usize = 0xB003;
const DESTROY_MUTEX_SYSCALL: usize = 0xB004;

#[derive(Debug)]
enum MutexError {
    NotFound,
}

pub fn system_call(
    code: usize,
    arg1: usize,
    _arg2: usize,
    _arg3: usize,
    _arg4: usize,
    _arg5: usize,
) -> base::error::Result<isize> {
    match code {
        CREATE_MUTEX_SYSCALL => {
            let mutex = Mutex::<ProcessTypes>::new();

            Ok(process::current_thread::<ProcessTypes>().lock(|thread| {
                thread
                    .process()
                    .lock(|process| process.descriptors_mut().insert_mutex(mutex))
            }))
        }
        DESTROY_MUTEX_SYSCALL => {
            process::current_thread::<ProcessTypes>().lock(|thread| {
                thread
                    .process()
                    .lock(|process| process.descriptors_mut().remove_mutex(arg1 as isize))
            });

            Ok(0)
        }
        LOCK_MUTEX_SYSCALL => {
            match process::current_thread::<ProcessTypes>().lock(|thread| {
                thread.process().lock(|process| {
                    process
                        .descriptors()
                        .mutex(arg1 as isize)
                        .map(|mutex| mutex.clone())
                })
            }) {
                Some(mutex) => mutex.lock(),
                None => return Err(Box::new(MutexError::NotFound)),
            };
            Ok(0)
        }
        TRY_LOCK_MUTEX_SYSCALL => {
            match process::current_thread::<ProcessTypes>().lock(|thread| {
                thread.process().lock(|process| {
                    process
                        .descriptors()
                        .mutex(arg1 as isize)
                        .map(|mutex| mutex.clone())
                })
            }) {
                Some(mutex) => Ok(mutex.try_lock() as isize),
                None => return Err(Box::new(MutexError::NotFound)),
            }
        }
        UNLOCK_MUTEX_SYSCALL => {
            match process::current_thread::<ProcessTypes>().lock(|thread| {
                thread.process().lock(|process| {
                    process
                        .descriptors()
                        .mutex(arg1 as isize)
                        .map(|mutex| mutex.clone())
                })
            }) {
                Some(mutex) => mutex.unlock(),
                None => return Err(Box::new(MutexError::NotFound)),
            };
            Ok(0)
        }
        _ => {
            log_error!("Invalid mutex system call: {}", code);
            Err(Box::new(SystemCallError::InvalidCode))
        }
    }
}

impl base::error::Error for MutexError {
    fn module_number(&self) -> i32 {
        SYSTEM_CALLS_MODULE_NUMBER
    }

    fn error_number(&self) -> base::error::Status {
        match self {
            MutexError::NotFound => base::error::Status::NotFound,
        }
    }
}

impl core::fmt::Display for MutexError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            MutexError::NotFound => write!(f, "Mutex not found"),
        }
    }
}
