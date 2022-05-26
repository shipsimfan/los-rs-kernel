use crate::SystemCallError;
use alloc::boxed::Box;
use base::log_error;

const GET_MEMORY_USAGE_SYSCALL: usize = 0x7000;

pub fn system_call(
    code: usize,
    arg1: usize,
    _arg2: usize,
    _arg3: usize,
    _arg4: usize,
    _arg5: usize,
) -> base::error::Result<isize> {
    match code {
        GET_MEMORY_USAGE_SYSCALL => {
            let target = super::to_ptr_mut(arg1)?;

            unsafe { *target = memory::get_memory_usage() }

            Ok(0)
        }
        _ => {
            log_error!("Invalid memory system call: {}", code);
            Err(Box::new(SystemCallError::InvalidCode))
        }
    }
}
