use crate::{
    error, logln, process,
    session::{console::Color, SubSession},
};

const CONSOLE_WRITE_SYSCALL: usize = 0x3000;
const CONSOLE_WRITE_STR_SYSCALL: usize = 0x3001;
const CONSOLE_CLEAR_SYSCALL: usize = 0x3002;
const CONSOLE_SET_ATTRIBUTE_SYSCALL: usize = 0x3003;
const CONSOLE_SET_FOREGROUND_COLOR_SYSCALL: usize = 0x3004;
const CONSOLE_SET_FOREGROUND_COLOR_RGB_SYSCALL: usize = 0x3005;
const CONSOLE_SET_BACKGROUND_COLOR_SYSCALL: usize = 0x3006;
const CONSOLE_SET_BACKGROUND_COLOR_RGB_SYSCALL: usize = 0x3007;
const CONSOLE_SET_CURSOR_POS_SYSCALL: usize = 0x3008;
const CONSOLE_GET_WIDTH: usize = 0x3009;
const CONSOLE_GET_HEIGHT: usize = 0x300A;

pub fn system_call(
    code: usize,
    arg1: usize,
    arg2: usize,
    arg3: usize,
    _arg4: usize,
    _arg5: usize,
) -> isize {
    let session_lock = match process::get_current_thread_mut()
        .get_process_mut()
        .get_session_mut()
    {
        Some(session) => session,
        None => return error::Status::InvalidSession as isize,
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
        CONSOLE_SET_ATTRIBUTE_SYSCALL => console_session.set_attribute(arg1),
        CONSOLE_SET_FOREGROUND_COLOR_SYSCALL => {
            console_session.set_foreground_color(Color::from_usize(arg1))
        }
        CONSOLE_SET_FOREGROUND_COLOR_RGB_SYSCALL => {
            console_session.set_foreground_color(Color::new(
                (arg1 & 0xFF) as u8,
                (arg2 & 0xFF) as u8,
                (arg3 & 0xFF) as u8,
            ))
        }
        CONSOLE_SET_BACKGROUND_COLOR_SYSCALL => {
            console_session.set_background_color(Color::from_usize(arg1))
        }
        CONSOLE_SET_BACKGROUND_COLOR_RGB_SYSCALL => {
            console_session.set_background_color(Color::new(
                (arg1 & 0xFF) as u8,
                (arg2 & 0xFF) as u8,
                (arg3 & 0xFF) as u8,
            ))
        }
        CONSOLE_SET_CURSOR_POS_SYSCALL => console_session.set_cursor_pos(arg1, arg2),
        CONSOLE_GET_WIDTH => {
            return match console_session.get_width() {
                Ok(width) => width,
                Err(status) => status.to_return_code(),
            }
        }
        CONSOLE_GET_HEIGHT => {
            return match console_session.get_height() {
                Ok(height) => height,
                Err(status) => status.to_return_code(),
            }
        }
        _ => {
            logln!("Invalid console system call: {}", code);
            Err(error::Status::InvalidRequestCode)
        }
    } {
        Ok(()) => 0,
        Err(status) => status.to_return_code(),
    }
}
