use crate::{error, logln, process, time};

const GET_PROCESS_TIME_SYSCALL: usize = 0x5000;
const GET_TIMEZONE_SYSCALL: usize = 0x5001;
const GET_EPOCH_TIME_SYSCALL: usize = 0x5002;
const SET_ALARM_SYSCALL: usize = 0x5003;
const SLEEP_SYSCALL: usize = 0x5004;

pub fn system_call(
    code: usize,
    arg1: usize,
    _arg2: usize,
    _arg3: usize,
    _arg4: usize,
    _arg5: usize,
) -> isize {
    match code {
        GET_PROCESS_TIME_SYSCALL => process::get_current_thread().process().unwrap().get_time(),
        GET_TIMEZONE_SYSCALL => time::get_timezone(),
        GET_EPOCH_TIME_SYSCALL => time::get_epoch_time(),
        SET_ALARM_SYSCALL => {
            time::set_alarm(arg1);
            0
        }
        SLEEP_SYSCALL => {
            time::sleep(arg1);
            0
        }
        _ => {
            logln!("Invalid thread system call: {}", code);
            error::Status::InvalidRequestCode.to_return_code()
        }
    }
}
