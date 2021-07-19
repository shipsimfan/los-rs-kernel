use core::fmt;

#[derive(Debug, PartialEq, Eq)]
#[repr(isize)]
#[allow(dead_code)]
pub enum Status {
    Success = 0,
    NotFound = -1,
    InvalidArgument = -2,
    AlreadyExists = -3,
    NotSupported = -4,
    DeviceError = -5,
    Timeout = -6,
    NoSession = -7,
    InvalidSystemCall = -8,
}

pub type Result = core::result::Result<(), Status>;

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Status::Success => "Success",
            Status::NotFound => "Not found",
            Status::InvalidArgument => "Invalid argument",
            Status::AlreadyExists => "Already exists",
            Status::NotSupported => "Not supported",
            Status::DeviceError => "Device error",
            Status::Timeout => "Timeout",
            Status::NoSession => "No session",
            Status::InvalidSystemCall => "Invalid system call",
        })
    }
}
