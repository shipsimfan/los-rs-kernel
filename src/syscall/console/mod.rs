use crate::{error, logln, memory::KERNEL_VMA, process, session::SubSession};

const CONSOLE_WRITE_SYSCALL: usize = 0x3000;
const CONSOLE_WRITE_STR_SYSCALL: usize = 0x3001;
const CONSOLE_CLEAR_SYSCALL: usize = 0x3002;

pub fn system_call(
    code: usize,
    arg1: usize,
    arg2: usize,
    _arg3: usize,
    _arg4: usize,
    _arg5: usize,
) -> usize {
    let console_session = match process::get_current_thread_mut()
        .get_process_mut()
        .get_session_mut()
    {
        Some(session) => match session.get_sub_session_mut() {
            SubSession::Console(console) => console,
        },
        None => return usize::MAX,
    };

    match match code {
        CONSOLE_WRITE_SYSCALL => {
            let c = (arg1 & 0xFF) as u8;
            console_session.write(&[c])
        }
        CONSOLE_WRITE_STR_SYSCALL => {
            if arg1 >= KERNEL_VMA || arg1 + arg2 >= KERNEL_VMA {
                Err(error::Status::InvalidArgument)
            } else {
                let slice = unsafe { core::slice::from_raw_parts(arg1 as *const u8, arg2) };
                let string = match alloc::str::from_utf8(slice) {
                    Ok(str) => str,
                    Err(_) => return usize::MAX,
                };
                console_session.write_str(string)
            }
        }
        CONSOLE_CLEAR_SYSCALL => console_session.clear(),
        _ => {
            logln!("Invalid console system call: {}", code);
            Ok(())
        }
    } {
        Ok(()) => 0,
        Err(_) => usize::MAX,
    }
}
