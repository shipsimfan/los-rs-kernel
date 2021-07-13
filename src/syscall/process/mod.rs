use crate::{logln, memory::KERNEL_VMA, process};

const WAIT_PROCESS_SYSCALL: usize = 0x0000;
const EXECUTE_SYSCALL: usize = 0x0001;

pub fn system_call(
    code: usize,
    arg1: usize,
    arg2: usize,
    _arg3: usize,
    _arg4: usize,
    _arg5: usize,
) -> usize {
    match code {
        WAIT_PROCESS_SYSCALL => process::wait_process(arg1),
        EXECUTE_SYSCALL => {
            if arg1 >= KERNEL_VMA || arg1 + arg2 >= KERNEL_VMA {
                usize::MAX
            } else {
                let slice = unsafe { core::slice::from_raw_parts(arg1 as *const u8, arg2) };
                match alloc::str::from_utf8(slice) {
                    Ok(filepath) => match process::execute(filepath) {
                        Ok(pid) => pid,
                        Err(_) => usize::MAX,
                    },
                    Err(_) => usize::MAX,
                }
            }
        }
        _ => {
            logln!("Invalid process system call: {}", code);
            usize::MAX
        }
    }
}
