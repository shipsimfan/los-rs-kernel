use crate::{process};

const CREATE_MUTEX_SYSCALL: usize = 0xA000;
const LOCK_MUTEX_SYSCALL: usize = 0xA001;
const TRY_LOCK_MUTEX_SYSCALL: usize = 0xA002;
const UNLOCK_MUTEX_SYSCALL: usize = 0xA003;
const DESTROY_MUTEX_SYSCALL: usize = 0xA004;

pub fn system_call(
    code: usize,
    arg1: usize,
    _arg2: usize,
    _arg3: usize,
    _arg4: usize,
    _arg5: usize,
) -> isize {
    match code {
        CREATE_MUTEX_SYSCALL => {
            match process::get_current_thread()
                .process()
                .unwrap()
                .create_mutex()
            {
                Ok(md) => md,
                Err(status) => status.to_return_code(),
            }
        }

        DESTROY_MUTEX_SYSCALL => {
            match process::get_current_thread()
                .process()
                .unwrap()
                .destroy_mutex(arg1)
            {
                Ok(md) => md,
                Err(status) => status.to_return_code(),
            }
        }
 
        LOCK_MUTEX_SYSCALL => {
            match process::get_current_thread()
                .process()
                .unwrap()
                .lock_mutex(arg1)
            0
        }

        TRY_LOCK_MUTEX_SYSCALL => {
            match process::get_current_thread()
                .process()
                .unwrap()
                .try_lock_mutex() as isize
        }
    }
}
