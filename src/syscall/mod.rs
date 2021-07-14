use crate::logln;

mod console;
mod event;
mod filesystem;
mod process;
mod stack;
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
