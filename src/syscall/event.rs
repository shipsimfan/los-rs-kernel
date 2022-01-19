use crate::{error, event::CEvent, logln, process, session::get_session_mut};

const PEEK_EVENT_SYSCALL: usize = 0x4000;

pub fn system_call(
    code: usize,
    arg1: usize,
    _arg2: usize,
    _arg3: usize,
    _arg4: usize,
    _arg5: usize,
) -> isize {
    match code {
        PEEK_EVENT_SYSCALL => match super::to_ptr_mut(arg1) {
            Ok(ptr) => {
                match process::get_current_thread()
                    .process()
                    .unwrap()
                    .session_id()
                {
                    Some(session_id) => match get_session_mut(session_id) {
                        Some(session) => match session.lock().peek_event() {
                            None => 0,
                            Some(event) => {
                                let cevent = CEvent::from(event);
                                unsafe { *ptr = cevent };
                                1
                            }
                        },
                        None => return error::Status::InvalidSession.to_return_code(),
                    },
                    None => return error::Status::InvalidSession.to_return_code(),
                }
            }
            Err(status) => status.to_return_code(),
        },
        _ => {
            logln!("Invalid process system call: {}", code);
            error::Status::InvalidRequestCode.to_return_code()
        }
    }
}
