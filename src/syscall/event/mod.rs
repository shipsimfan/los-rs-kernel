use crate::{error, event::CEvent, logln, process};

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
                match process::get_current_thread_mut()
                    .get_process_mut()
                    .get_session_mut()
                {
                    Some(session) => match session.peek_event() {
                        None => 0,
                        Some(event) => {
                            let cevent = CEvent::from(event);
                            unsafe { *ptr = cevent };
                            1
                        }
                    },
                    None => panic!("Attempting to peek event on daemon process!"),
                }
            }
            Err(status) => status as isize,
        },
        _ => {
            logln!("Invalid process system call: {}", code);
            error::Status::InvalidSystemCall as isize
        }
    }
}
