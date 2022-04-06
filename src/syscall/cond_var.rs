use crate::{error, logln, process};

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
) -> isize {
    match code {
        CREATE_COND_VAR_SYSCALL => {
            match process::get_current_thread()
                .process()
                .unwrap()
                .create_cond_var()
            {
                Ok(cond) => cond,
                Err(status) => status.to_return_code(),
            }
        }
        DESTROY_COND_VAR_SYSCALL => {
            process::get_current_thread()
                .process()
                .unwrap()
                .destroy_cond_var(arg1 as isize);
            0
        }
        WAIT_COND_VAR_SYSCALL => {
            let cond_var = match process::get_current_thread()
                .process()
                .unwrap()
                .get_cond_var(arg1 as isize)
            {
                Ok(cond_var) => cond_var,
                Err(error) => return error.to_return_code(),
            };
            cond_var.wait();
            0
        }
        SIGNAL_COND_VAR => {
            let cond_var = match process::get_current_thread()
                .process()
                .unwrap()
                .get_cond_var(arg1 as isize)
            {
                Ok(cond_var) => cond_var,
                Err(error) => return error.to_return_code(),
            };
            cond_var.singal();
            0
        }
        BROADCAST_COND_VAR => {
            let cond_var = match process::get_current_thread()
                .process()
                .unwrap()
                .get_cond_var(arg1 as isize)
            {
                Ok(cond_var) => cond_var,
                Err(error) => return error.to_return_code(),
            };
            cond_var.broadcast();
            0
        }
        _ => {
            logln!("Invalid userspace conditional variablt system call: {}", code);
            error::Status::InvalidRequestCode.to_return_code()
        }
    }
}
