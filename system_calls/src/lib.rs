#![no_std]

use alloc::boxed::Box;
use base::{error::SYSTEM_CALLS_MODULE_NUMBER, log_info};

mod cond_var;
mod console;
mod device;
mod event;
mod filesystem;
mod memory_calls;
mod mutex;
mod pipe;
mod process;
mod session;
mod signal;
mod thread;
mod time;

extern crate alloc;

trait NullTerminator: PartialEq {
    fn null_terminator() -> Self;
}

#[derive(Debug)]
enum SystemCallError {
    InvalidCode,
    ArgumentSecurity,
    InvalidUTF8,
}

const MODULE_NAME: &str = "System Calls";

pub fn handler(
    code: usize,
    arg1: usize,
    arg2: usize,
    arg3: usize,
    arg4: usize,
    arg5: usize,
    _: interrupts::Registers,
    _: u64,
    _: u64,
    _: u64,
) -> base::error::Result<isize> {
    if code <= 0x0FFF {
        process::system_call(code, arg1, arg2, arg3, arg4, arg5)
    } else if code <= 0x1FFF {
        thread::system_call(code, arg1, arg2, arg3, arg4, arg5)
    } else if code <= 0x2FFF {
        filesystem::system_call(code, arg1, arg2, arg3, arg4, arg5)
    } else if code <= 0x3FFF {
        console::system_call(code, arg1, arg2, arg3, arg4, arg5)
    } else if code <= 0x4FFF {
        event::system_call(code, arg1, arg2, arg3, arg4, arg5)
    } else if code <= 0x5FFF {
        time::system_call(code, arg1, arg2, arg3, arg4, arg5)
    } else if code <= 0x6FFF {
        device::system_call(code, arg1, arg2, arg3, arg4, arg5)
    } else if code <= 0x7FFF {
        memory_calls::system_call(code, arg1, arg2, arg3, arg4, arg5)
    } else if code <= 0x8FFF {
        session::system_call(code, arg1, arg2, arg3, arg4, arg5)
    } else if code <= 0x9FFF {
        signal::system_call(code, arg1, arg2, arg3, arg4, arg5)
    } else if code <= 0xAFFF {
        pipe::system_call(code, arg1, arg2, arg3, arg4, arg5)
    } else if code <= 0xBFFF {
        mutex::system_call(code, arg1, arg2, arg3, arg4, arg5)
    } else if code <= 0xCFFF {
        cond_var::system_call(code, arg1, arg2, arg3, arg4, arg5)
    } else {
        log_info!("Invalid system call: {}", code);
        Err(Box::new(SystemCallError::InvalidCode))
    }
}

// Argument translation functions
fn to_slice_mut<T>(ptr: usize, len: usize) -> base::error::Result<&'static mut [T]> {
    if ptr >= memory::KERNEL_VMA || ptr + len * core::mem::size_of::<T>() >= memory::KERNEL_VMA {
        Err(Box::new(SystemCallError::ArgumentSecurity))
    } else {
        Ok(unsafe { core::slice::from_raw_parts_mut(ptr as *mut T, len) })
    }
}

fn to_slice<T>(ptr: usize, len: usize) -> base::error::Result<&'static [T]> {
    if ptr >= memory::KERNEL_VMA || ptr + len * core::mem::size_of::<T>() >= memory::KERNEL_VMA {
        Err(Box::new(SystemCallError::ArgumentSecurity))
    } else {
        Ok(unsafe { core::slice::from_raw_parts(ptr as *const T, len) })
    }
}

fn to_slice_null<T: NullTerminator>(ptr: usize) -> base::error::Result<&'static [T]> {
    if ptr >= memory::KERNEL_VMA {
        return Err(Box::new(SystemCallError::ArgumentSecurity));
    }

    let mut p = ptr as *const T;
    let mut len = 0;
    while unsafe { *p != T::null_terminator() } {
        p = unsafe { p.add(1) };
        len += 1;
    }

    Ok(unsafe { core::slice::from_raw_parts(ptr as *const T, len) })
}

fn to_str(str: usize) -> base::error::Result<&'static str> {
    let slice = to_slice_null(str)?;
    match core::str::from_utf8(slice) {
        Ok(str) => Ok(str),
        Err(_) => Err(Box::new(SystemCallError::InvalidUTF8)),
    }
}

fn to_ptr_mut<T>(ptr: usize) -> base::error::Result<*mut T> {
    if ptr >= memory::KERNEL_VMA || ptr + core::mem::size_of::<T>() >= memory::KERNEL_VMA {
        Err(Box::new(SystemCallError::ArgumentSecurity))
    } else {
        Ok(ptr as *mut T)
    }
}

impl base::error::Error for SystemCallError {
    fn module_number(&self) -> i32 {
        SYSTEM_CALLS_MODULE_NUMBER
    }

    fn error_number(&self) -> base::error::Status {
        match self {
            SystemCallError::InvalidCode => base::error::Status::InvalidRequestCode,
            SystemCallError::ArgumentSecurity => base::error::Status::ArgumentSecurity,
            SystemCallError::InvalidUTF8 => base::error::Status::InvalidUTF8,
        }
    }
}

impl core::fmt::Display for SystemCallError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            SystemCallError::InvalidCode => write!(f, "Invalid system call"),
            SystemCallError::ArgumentSecurity => write!(f, "Arguments reach into kernel space"),
            SystemCallError::InvalidUTF8 => write!(f, "Invalid UTF-8"),
        }
    }
}

// Null terminator implementations
impl NullTerminator for u8 {
    fn null_terminator() -> Self {
        0
    }
}

impl NullTerminator for i8 {
    fn null_terminator() -> Self {
        0
    }
}

impl NullTerminator for u16 {
    fn null_terminator() -> Self {
        0
    }
}

impl NullTerminator for i16 {
    fn null_terminator() -> Self {
        0
    }
}

impl NullTerminator for u32 {
    fn null_terminator() -> Self {
        0
    }
}

impl NullTerminator for i32 {
    fn null_terminator() -> Self {
        0
    }
}

impl NullTerminator for u64 {
    fn null_terminator() -> Self {
        0
    }
}

impl NullTerminator for i64 {
    fn null_terminator() -> Self {
        0
    }
}

impl NullTerminator for usize {
    fn null_terminator() -> Self {
        0
    }
}

impl NullTerminator for isize {
    fn null_terminator() -> Self {
        0
    }
}

impl NullTerminator for char {
    fn null_terminator() -> Self {
        '\0'
    }
}

impl<T> NullTerminator for *const T {
    fn null_terminator() -> Self {
        core::ptr::null()
    }
}

impl<T> NullTerminator for *mut T {
    fn null_terminator() -> Self {
        core::ptr::null_mut()
    }
}
