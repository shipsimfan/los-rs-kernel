use crate::SystemCallError;
use alloc::{boxed::Box, string::String};
use base::{error::SYSTEM_CALLS_MODULE_NUMBER, log_error};
use process_types::ProcessTypes;
use sessions::{Color, ConsoleOutputDevice};

#[derive(Debug)]
enum ConsoleError {
    NotAConsoleSession,
}

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

pub fn system_call(
    code: usize,
    arg1: usize,
    arg2: usize,
    arg3: usize,
    _arg4: usize,
    _arg5: usize,
) -> base::error::Result<isize> {
    let session = process::current_thread::<ProcessTypes>()
        .lock(|thread| thread.process().lock(|process| process.owner().clone()));

    let console_output = session.lock(
        |session| -> base::error::Result<ConsoleOutputDevice<ProcessTypes>> {
            match session.as_console() {
                Some(console) => Ok(console.get_output_device()),
                None => return Err(Box::new(ConsoleError::NotAConsoleSession)),
            }
        },
    )?;

    match code {
        CONSOLE_WRITE_CH_SYSCALL => {
            let c = (arg1 & 0xFF) as u8;
            console_output.write(&[c]).map(|ret| ret as isize)
        }
        CONSOLE_WRITE_STR_SYSCALL => console_output
            .write_str(super::to_str(arg1)?)
            .map(|ret| ret as isize),
        CONSOLE_CLEAR_SYSCALL => console_output.clear().map(|_| 0),
        CONSOLE_SET_ATTRIBUTE_SYSCALL => console_output.set_attribute(arg1).map(|_| 0),
        CONSOLE_SET_FOREGROUND_COLOR_SYSCALL => console_output
            .set_foreground_color(Color::from_usize(arg1))
            .map(|_| 0),
        CONSOLE_SET_FOREGROUND_COLOR_RGB_SYSCALL => console_output
            .set_foreground_color(Color::new(
                (arg1 & 0xFF) as u8,
                (arg2 & 0xFF) as u8,
                (arg3 & 0xFF) as u8,
            ))
            .map(|_| 0),
        CONSOLE_SET_BACKGROUND_COLOR_SYSCALL => console_output
            .set_background_color(Color::from_usize(arg1))
            .map(|_| 0),
        CONSOLE_SET_BACKGROUND_COLOR_RGB_SYSCALL => console_output
            .set_background_color(Color::new(
                (arg1 & 0xFF) as u8,
                (arg2 & 0xFF) as u8,
                (arg3 & 0xFF) as u8,
            ))
            .map(|_| 0),
        CONSOLE_SET_CURSOR_POS_SYSCALL => console_output.set_cursor_pos(arg1, arg2).map(|_| 0),
        CONSOLE_GET_WIDTH => console_output.get_width(),
        CONSOLE_GET_HEIGHT => console_output.get_height(),
        CONSOLE_WRITE_SYSCALL => console_output
            .write_str(&String::from_utf8_lossy(super::to_slice_mut(arg1, arg2)?))
            .map(|ret| ret as isize),
        CONSOLE_SET_CURSOR_STATE => console_output.set_cursor_state(arg1 != 0).map(|_| 0),
        _ => {
            log_error!("Invalid console system call: {}", code);
            Err(Box::new(SystemCallError::InvalidCode))
        }
    }
}

impl base::error::Error for ConsoleError {
    fn module_number(&self) -> i32 {
        SYSTEM_CALLS_MODULE_NUMBER
    }

    fn error_number(&self) -> base::error::Status {
        match self {
            ConsoleError::NotAConsoleSession => base::error::Status::InvalidSession,
        }
    }
}

impl core::fmt::Display for ConsoleError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            ConsoleError::NotAConsoleSession => {
                write!(f, "Using console system calls on a non-console session")
            }
        }
    }
}
