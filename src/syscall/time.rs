use crate::{error, logln, process, time};

const GET_PROCESS_TIME_SYSCALL: usize = 0x5000;
const GET_TIMEZONE_SYSCALL: usize = 0x5001;
const GET_EPOCH_TIME_SYSCALL: usize = 0x5002;

pub fn system_call(
    code: usize,
    _arg1: usize,
    _arg2: usize,
    _arg3: usize,
    _arg4: usize,
    _arg5: usize,
) -> isize {
    match code {
        GET_PROCESS_TIME_SYSCALL => process::get_current_thread().process().unwrap().get_time(),
        GET_TIMEZONE_SYSCALL => time::get_timezone(),
        GET_EPOCH_TIME_SYSCALL => time::get_epoch_time(),
        _ => {
            logln!("Invalid time system call: {}", code);
            error::Status::InvalidRequestCode.to_return_code()
        }
    }
}
