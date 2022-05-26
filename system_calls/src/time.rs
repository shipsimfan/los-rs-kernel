use crate::SystemCallError;
use alloc::boxed::Box;
use base::log_error;
use process_types::ProcessTypes;

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
) -> base::error::Result<isize> {
    match code {
        GET_PROCESS_TIME_SYSCALL => Ok(process::current_thread::<ProcessTypes>()
            .lock(|thread| thread.process().lock(|process| process.time()))),
        GET_TIMEZONE_SYSCALL => Ok(time::get_timezone()),
        GET_EPOCH_TIME_SYSCALL => Ok(time::get_epoch_time()),
        SET_ALARM_SYSCALL => {
            time::set_alarm::<ProcessTypes>(arg1);
            Ok(0)
        }
        SLEEP_SYSCALL => {
            time::sleep::<ProcessTypes>(arg1);
            Ok(0)
        }
        _ => {
            log_error!("Invalid time system call: {}", code);
            Err(Box::new(SystemCallError::InvalidCode))
        }
    }
}
