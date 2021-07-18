use crate::{
    error, logln,
    memory::{KERNEL_VMA, PAGE_SIZE},
};

mod console;
mod event;
mod filesystem;
mod process;
mod thread;

trait NullTerminator: PartialEq {
    fn null_terminator() -> Self;
}

#[no_mangle]
extern "C" fn system_call(
    code: usize,
    arg1: usize,
    arg2: usize,
    arg3: usize,
    arg4: usize,
    arg5: usize,
) -> usize {
    if code <= 0x0FFF {
        process::system_call(code, arg1, arg2, arg3, arg4, arg5)
    } else if code >= 0x1000 && code <= 0x1FFF {
        thread::system_call(code, arg1, arg2, arg3, arg4, arg5)
    } else if code >= 0x2000 && code <= 0x2FFF {
        filesystem::system_call(code, arg1, arg2, arg3, arg4, arg5)
    } else if code >= 0x3000 && code <= 0x3FFF {
        console::system_call(code, arg1, arg2, arg3, arg4, arg5)
    } else if code >= 0x4000 && code <= 0x4FFF {
        event::system_call(code, arg1, arg2, arg3, arg4, arg5)
    } else {
        logln!("Invalid system call: {}", code);
        usize::MAX
    }
}

// Argument translation functions
fn to_slice_mut(ptr: usize, len: usize) -> Result<&'static mut [u8], error::Status> {
    if ptr < PAGE_SIZE || ptr >= KERNEL_VMA || ptr + len >= KERNEL_VMA {
        Err(error::Status::InvalidArgument)
    } else {
        Ok(unsafe { core::slice::from_raw_parts_mut(ptr as *mut u8, len) })
    }
}

fn to_slice_null<T: NullTerminator>(ptr: usize) -> Result<&'static [T], error::Status> {
    if ptr >= KERNEL_VMA {
        return Err(error::Status::InvalidArgument);
    }

    let mut p = ptr as *const T;
    let mut len = 0;
    while unsafe { *p != T::null_terminator() } {
        p = unsafe { p.add(1) };
        len += 1;
    }

    Ok(unsafe { core::slice::from_raw_parts(ptr as *const T, len) })
}

fn to_str(str: usize) -> Result<&'static str, error::Status> {
    let slice = to_slice_null(str)?;
    match core::str::from_utf8(slice) {
        Ok(str) => Ok(str),
        Err(_) => Err(error::Status::InvalidArgument),
    }
}

fn to_ptr_mut<T>(ptr: usize) -> Result<*mut T, error::Status> {
    if ptr < PAGE_SIZE || ptr >= KERNEL_VMA || ptr + core::mem::size_of::<T>() >= KERNEL_VMA {
        Err(error::Status::InvalidArgument)
    } else {
        Ok(ptr as *mut T)
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
