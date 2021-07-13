use crate::logln;

pub fn system_call(
    code: usize,
    _arg1: usize,
    _arg2: usize,
    _arg3: usize,
    _arg4: usize,
    _arg5: usize,
) -> usize {
    match code {
        _ => {
            logln!("Invalid filesystem system call: {}", code);
            usize::MAX
        }
    }
}
