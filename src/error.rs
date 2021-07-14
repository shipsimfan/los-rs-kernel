use core::fmt;

#[derive(Debug, PartialEq, Eq)]
pub enum Status {
    NotFound,
    InvalidArgument,
    AlreadyExists,
    NotSupported,
    DeviceError,
    Timeout,
}

pub type Result = core::result::Result<(), Status>;

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Status::NotFound => "Not found",
            Status::InvalidArgument => "Invalid argument",
            Status::AlreadyExists => "Already exists",
            Status::NotSupported => "Not supported",
            Status::DeviceError => "Device error",
            Status::Timeout => "Timeout",
        })
    }
}
