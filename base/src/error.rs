pub trait Error: core::fmt::Display + core::fmt::Debug {
    fn to_standard(&self) -> StandardError;
}

#[repr(usize)]
pub enum StandardError {
    Success = 0,
    ProcessNotFound,
    ThreadNotFound,
}

impl core::fmt::Debug for StandardError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        use StandardError::*;

        write!(
            f,
            "{}",
            match self {
                Success => "Success",
                ProcessNotFound => "Process not found",
                ThreadNotFound => "Thread not found",
            }
        )
    }
}
