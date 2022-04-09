use alloc::string::String;

use crate::{
    error, logln, process,
    session::{console::Color, get_session, SubSession},
};

const CONSOLE_WRITE_CH_SYSCALL: usize = 0x3000;
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
const CONSOLE_WRITE_SYSCALL: usize = 0x300B;
const CONSOLE_SET_CURSOR_STATE: usize = 0x300C;

extern "C" {
    static mut LOCAL_CRITICAL_COUNT: usize;
}

pub fn system_call(
    code: usize,
    arg1: usize,
    arg2: usize,
    arg3: usize,
    _arg4: usize,
    _arg5: usize,
) -> isize {
    let session = match process::get_current_thread()
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

    let mut console_output = match session.lock().get_sub_session() {
        SubSession::Console(console) => console.get_output_device(),
    };

    let ret = match match code {
        CONSOLE_WRITE_CH_SYSCALL => {
            let c = (arg1 & 0xFF) as u8;
            console_output.write(&[c])
        }
        CONSOLE_WRITE_STR_SYSCALL => match super::to_str(arg1) {
            Ok(str) => console_output.write_str(str),
            Err(status) => Err(status),
        },
        CONSOLE_CLEAR_SYSCALL => console_output.clear(),
        CONSOLE_SET_ATTRIBUTE_SYSCALL => console_output.set_attribute(arg1),
        CONSOLE_SET_FOREGROUND_COLOR_SYSCALL => {
            console_output.set_foreground_color(Color::from_usize(arg1))
        }
        CONSOLE_SET_FOREGROUND_COLOR_RGB_SYSCALL => {
            console_output.set_foreground_color(Color::new(
                (arg1 & 0xFF) as u8,
                (arg2 & 0xFF) as u8,
                (arg3 & 0xFF) as u8,
            ))
        }
        CONSOLE_SET_BACKGROUND_COLOR_SYSCALL => {
            console_output.set_background_color(Color::from_usize(arg1))
        }
        CONSOLE_SET_BACKGROUND_COLOR_RGB_SYSCALL => {
            console_output.set_background_color(Color::new(
                (arg1 & 0xFF) as u8,
                (arg2 & 0xFF) as u8,
                (arg3 & 0xFF) as u8,
            ))
        }
        CONSOLE_SET_CURSOR_POS_SYSCALL => console_output.set_cursor_pos(arg1, arg2),
        CONSOLE_GET_WIDTH => {
            return match console_output.get_width() {
                Ok(width) => width,
                Err(status) => status.to_return_code(),
            }
        }
        CONSOLE_GET_HEIGHT => {
            return match console_output.get_height() {
                Ok(height) => height,
                Err(status) => status.to_return_code(),
            }
        }
        CONSOLE_WRITE_SYSCALL => match super::to_slice_mut(arg1, arg2) {
            Ok(slice) => console_output.write_str(&String::from_utf8_lossy(slice)),
            Err(status) => Err(status),
        },
        CONSOLE_SET_CURSOR_STATE => console_output.set_cursor_state(arg1 != 0),
        _ => {
            logln!("Invalid console system call: {}", code);
            Err(error::Status::InvalidRequestCode)
        }
    } {
        Ok(()) => 0,
        Err(status) => status.to_return_code(),
    };

    let _c = unsafe { LOCAL_CRITICAL_COUNT };

    ret
}
