use crate::{logln, memory::KERNEL_VMA, process, session::CEvent};

const PEEK_EVENT_SYSCALL: usize = 0x4000;

pub fn system_call(
    code: usize,
    arg1: usize,
    _arg2: usize,
    _arg3: usize,
    _arg4: usize,
    _arg5: usize,
) -> usize {
    match code {
        PEEK_EVENT_SYSCALL => {
            if arg1 >= KERNEL_VMA || arg1 + core::mem::size_of::<CEvent>() >= KERNEL_VMA {
                0
            } else {
                match process::get_current_thread_mut()
                    .get_process_mut()
                    .get_session_mut()
                {
                    Some(session) => match session.peek_event() {
                        None => 0,
                        Some(event) => {
                            let cevent = CEvent::from(event);
                            let ptr = arg1 as *mut CEvent;
                            unsafe { *ptr = cevent };
                            1
                        }
                    },
                    None => panic!("Attempting to peek event on daemon process!"),
                }
            }
        }
        _ => {
            logln!("Invalid process system call: {}", code);
            usize::MAX
        }
    }
}
