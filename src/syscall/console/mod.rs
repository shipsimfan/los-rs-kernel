use crate::{error, logln, process, session::SubSession};

const CONSOLE_WRITE_SYSCALL: usize = 0x3000;
const CONSOLE_WRITE_STR_SYSCALL: usize = 0x3001;
const CONSOLE_CLEAR_SYSCALL: usize = 0x3002;

pub fn system_call(
    code: usize,
    arg1: usize,
    _arg2: usize,
    _arg3: usize,
    _arg4: usize,
    _arg5: usize,
) -> isize {
    let session_lock = match process::get_current_thread_mut()
        .get_process_mut()
        .get_session_mut()
    {
        Some(session) => session,
        None => return error::Status::NoSession as isize,
    };

    let mut session = session_lock.lock();
    let console_session = match session.get_sub_session_mut() {
        SubSession::Console(console) => console,
    };

    match match code {
        CONSOLE_WRITE_SYSCALL => {
            let c = (arg1 & 0xFF) as u8;
            console_session.write(&[c])
        }
        CONSOLE_WRITE_STR_SYSCALL => match super::to_str(arg1) {
            Ok(str) => console_session.write_str(str),
            Err(status) => Err(status),
        },
        CONSOLE_CLEAR_SYSCALL => console_session.clear(),
        _ => {
            logln!("Invalid console system call: {}", code);
            Err(error::Status::InvalidSystemCall)
        }
    } {
        Ok(()) => 0,
        Err(status) => status as isize,
    }
}
