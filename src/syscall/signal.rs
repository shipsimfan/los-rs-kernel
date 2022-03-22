use crate::{error, ipc::SignalHandler, logln, process, session};

const RAISE_SESSION_SYSCALL: usize = 0x9000;
const RAISE_PROCESS_SYSCALL: usize = 0x9001;
const RAISE_SELF_SYSCALL: usize = 0x9002;
const SET_SIGNAL_TYPE_SYSCALL: usize = 0x9003;
const MASK_SIGNAL_SYSCALL: usize = 0x9004;
const UNMASK_SIGNAL_SYSCALL: usize = 0x9005;

pub fn system_call(
    code: usize,
    arg1: usize,
    arg2: usize,
    arg3: usize,
    _arg4: usize,
    _arg5: usize,
) -> isize {
    match code {
        RAISE_SESSION_SYSCALL => {
            let process = match arg1 {
                0 => match process::get_daemon_process((arg2 & 0x7FFFFFFFFFFF) as isize) {
                    Some(process) => process,
                    None => return error::Status::NoProcess.to_return_code(),
                },
                _ => {
                    let session = match session::get_session((arg1 & 0x7FFFFFFFFFFF) as isize) {
                        Some(session) => session,
                        None => return error::Status::InvalidSession.to_return_code(),
                    };

                    let session = session.lock();
                    match session.get_process((arg2 & 0x7FFFFFFFFFFF) as isize) {
                        Some(process) => process,
                        None => return error::Status::NoProcess.to_return_code(),
                    }
                }
            };

            process.raise((arg3 & 0xFF) as u8);
            0
        }
        RAISE_PROCESS_SYSCALL => {
            let process = match process::get_current_thread()
                .process()
                .unwrap()
                .session_id()
            {
                Some(session_id) => match session::get_session(session_id) {
                    Some(session) => {
                        match session.lock().get_process((arg1 & 0x7FFFFFFFFFFF) as isize) {
                            Some(process) => process,
                            None => return error::Status::NoProcess.to_return_code(),
                        }
                    }
                    None => return error::Status::InvalidSession.to_return_code(),
                },
                None => match process::get_daemon_process((arg1 & 0x7FFFFFFFFFFF) as isize) {
                    Some(process) => process,
                    None => return error::Status::NoProcess.to_return_code(),
                },
            };

            process.raise((arg2 & 0xFF) as u8);
            0
        }
        RAISE_SELF_SYSCALL => {
            process::get_current_thread()
                .process()
                .unwrap()
                .raise((arg1 & 0xFF) as u8);
            0
        }
        SET_SIGNAL_TYPE_SYSCALL => {
            let handler = match SignalHandler::from(arg2) {
                Some(handler) => handler,
                None => return 0,
            };

            process::get_current_thread()
                .process()
                .unwrap()
                .set_signal_handler((arg1 & 0xFF) as u8, handler);
            0
        }
        MASK_SIGNAL_SYSCALL => {
            process::get_current_thread()
                .process()
                .unwrap()
                .set_signal_mask((arg1 & 0xFF) as u8, true);
            0
        }
        UNMASK_SIGNAL_SYSCALL => {
            process::get_current_thread()
                .process()
                .unwrap()
                .set_signal_mask((arg1 & 0xFF) as u8, false);
            0
        }
        _ => {
            logln!("Invalid signal system call: {}", code);
            error::Status::InvalidRequestCode.to_return_code()
        }
    }
}
