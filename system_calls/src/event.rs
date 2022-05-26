use alloc::boxed::Box;
use base::{error::SYSTEM_CALLS_MODULE_NUMBER, log_error};
use process::CurrentQueue;
use process_types::ProcessTypes;
use sessions::Event;

use crate::SystemCallError;

const PEEK_EVENT_SYSCALL: usize = 0x4000;
const POLL_EVENT_SYSCALL: usize = 0x4001;

enum EventResult {
    None(Option<CurrentQueue<ProcessTypes>>),
    Some(Event),
}

#[derive(Debug)]
enum EventError {
    InvalidSession,
    Interrupted,
}

#[repr(C)]
pub struct CEvent {
    class: usize,
    param1: usize,
    param2: usize,
}

pub fn system_call(
    code: usize,
    arg1: usize,
    _arg2: usize,
    _arg3: usize,
    _arg4: usize,
    _arg5: usize,
) -> base::error::Result<isize> {
    match code {
        PEEK_EVENT_SYSCALL | POLL_EVENT_SYSCALL => {
            let ptr = super::to_ptr_mut(arg1)?;

            let session = process::current_thread::<ProcessTypes>()
                .lock(|thread| thread.process().lock(|process| process.owner().clone()));

            let event = {
                match session.lock(|session| match session.peek_event() {
                    None => EventResult::None(session.get_event_thread_queue()),
                    Some(event) => EventResult::Some(event),
                }) {
                    EventResult::None(queue) => match code == PEEK_EVENT_SYSCALL {
                        true => return Ok(0),
                        false => {
                            let queue = match queue {
                                Some(queue) => queue,
                                None => return Err(Box::new(EventError::InvalidSession)),
                            };

                            loop {
                                process::current_thread::<ProcessTypes>()
                                    .lock(|thread| thread.set_signal_interruptable());
                                process::yield_thread(Some(queue.clone()));

                                if process::current_thread::<ProcessTypes>()
                                    .lock(|thread| thread.signal_interrupted())
                                {
                                    return Err(Box::new(EventError::Interrupted));
                                }

                                match session.lock(|session| session.peek_event()) {
                                    Some(event) => break event,
                                    None => {}
                                }
                            }
                        }
                    },
                    EventResult::Some(event) => event,
                }
            };

            let cevent = CEvent::from(event);
            unsafe { *ptr = cevent };
            Ok(1)
        }
        _ => {
            log_error!("Invalid event system call: {}", code);
            Err(Box::new(SystemCallError::InvalidCode))
        }
    }
}

impl base::error::Error for EventError {
    fn module_number(&self) -> i32 {
        SYSTEM_CALLS_MODULE_NUMBER
    }

    fn error_number(&self) -> base::error::Status {
        match self {
            EventError::InvalidSession => base::error::Status::InvalidSession,
            EventError::Interrupted => base::error::Status::Interrupted,
        }
    }
}

impl core::fmt::Display for EventError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            EventError::InvalidSession => {
                write!(f, "Session does not have an event queue")
            }
            EventError::Interrupted => write!(f, "Interrupted while polling for event"),
        }
    }
}

impl From<Event> for CEvent {
    fn from(event: Event) -> Self {
        let (class, param1, param2) = match event {
            Event::KeyPress(key, state) => (0, key as usize, state.into()),
            Event::KeyRelease(key, state) => (1, key as usize, state.into()),
        };

        CEvent {
            class,
            param1,
            param2,
        }
    }
}
