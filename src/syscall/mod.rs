use crate::{
    error, logln,
    memory::{KERNEL_VMA, PAGE_SIZE},
};

mod console;
mod event;
mod filesystem;
mod process;
mod thread;

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
fn to_slice(ptr: usize, len: usize) -> Result<&'static [u8], error::Status> {
    if ptr < PAGE_SIZE || ptr >= KERNEL_VMA || ptr + len >= KERNEL_VMA {
        Err(error::Status::InvalidArgument)
    } else {
        Ok(unsafe { core::slice::from_raw_parts(ptr as *const u8, len) })
    }
}

fn to_slice_mut(ptr: usize, len: usize) -> Result<&'static mut [u8], error::Status> {
    if ptr < PAGE_SIZE || ptr >= KERNEL_VMA || ptr + len >= KERNEL_VMA {
        Err(error::Status::InvalidArgument)
    } else {
        Ok(unsafe { core::slice::from_raw_parts_mut(ptr as *mut u8, len) })
    }
}

fn to_str(str: usize, len: usize) -> Result<&'static str, error::Status> {
    let slice = to_slice(str, len)?;
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
