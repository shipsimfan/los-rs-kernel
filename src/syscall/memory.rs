use crate::{error, logln, memory};

const GET_MEMORY_USAGE_SYSCALL: usize = 0x7000;

pub fn system_call(
    code: usize,
    arg1: usize,
    _arg2: usize,
    _arg3: usize,
    _arg4: usize,
    _arg5: usize,
) -> isize {
    match code {
        GET_MEMORY_USAGE_SYSCALL => {
            let target = match super::to_ptr_mut(arg1) {
                Ok(target) => target,
                Err(status) => return status.to_return_code(),
            };

            unsafe { *target = memory::get_memory_usage() }

            0
        }
        _ => {
            logln!("Invalid device system call: {}", code);
            error::Status::InvalidRequestCode.to_return_code()
        }
    }
}
