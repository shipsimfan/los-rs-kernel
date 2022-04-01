use crate::{error, event::CEvent, logln, process, session::get_session};

const PEEK_EVENT_SYSCALL: usize = 0x4000;
const POLL_EVENT_SYSCALL: usize = 0x4001;

pub fn system_call(
    code: usize,
    arg1: usize,
    _arg2: usize,
    _arg3: usize,
    _arg4: usize,
    _arg5: usize,
) -> isize {
    match code {
        PEEK_EVENT_SYSCALL | POLL_EVENT_SYSCALL => match super::to_ptr_mut(arg1) {
            Ok(ptr) => {
                let session_lock = match process::get_current_thread()
                    .process()
                    .unwrap()
                    .session_id()
                {
                    Some(session_id) => match get_session(session_id) {
                        Some(session) => session,
                        None => return error::Status::InvalidSession.to_return_code(),
                    },
                    None => return error::Status::InvalidSession.to_return_code(),
                };

                let event = {
                    let mut session = session_lock.lock();
                    match session.peek_event() {
                        None => match code == PEEK_EVENT_SYSCALL {
                            true => return 0,
                            false => loop {
                                let queue = session.get_event_thread_queue();
                                drop(session);
                                process::get_current_thread().set_signal_interruptable();
                                process::yield_thread(Some(queue), None);

                                if process::get_current_thread().signal_interrupted() {
                                    return error::Status::Interrupted.to_return_code();
                                }

                                session = session_lock.lock();
                                match session.peek_event() {
                                    Some(event) => break event,
                                    None => {}
                                }
                            },
                        },
                        Some(event) => event,
                    }
                };

                let cevent = CEvent::from(event);
                unsafe { *ptr = cevent };
                1
            }
            Err(status) => status.to_return_code(),
        },
        _ => {
            logln!("Invalid process system call: {}", code);
            error::Status::InvalidRequestCode.to_return_code()
        }
    }
}
