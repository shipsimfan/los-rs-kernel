use core::fmt;

#[derive(Debug, PartialEq, Eq)]
#[repr(usize)]
#[allow(dead_code)]
pub enum Status {
    Success,                 // EOK
    OutOfDomain,             // EDOM
    OutOfRange,              // ERANGE
    PermissionDenied,        // EACCESS
    InUse,                   // EINUSE
    NotAvailable,            // ENOTAVAILABLE
    NotSupported,            // ENOTSUP
    TryAgain,                // EAGAIN
    AlreadyInProgress,       // EALREADY
    BadDescriptor,           // EBADD
    BadMessage,              // EBADMSG
    InvalidRequestCode,      // EBADRQC
    Busy,                    // EBUSY
    Cancelled,               // ECANCELLED
    Exists,                  // EEXIST
    BadAddress,              // EFAULT
    TooBig,                  // E2BIG
    InvalidUTF8,             // EILSEQ
    InProgress,              // EINPROGRESS
    InvalidArgument,         // EINVALIDARGUMENT
    IOError,                 // EIO
    IsDirectory,             // EISDIR
    NameTooLong,             // ENAMETOOLONG
    NoDevice,                // ENODEV
    NoEntry,                 // ENOENT
    InvalidExecutableFormat, // ENOEXEC
    NoSpace,                 // ENOSPC
    NotImplemented,          // ENOSYS
    NotDirectory,            // ENOTDIR
    NotEmpty,                // ENOTEMPTY
    InvalidIOCtrl,           // ENOTTY
    NotUnique,               // ENOTUNIQ
    ReadOnlyFilesystem,      // EROFS
    NoProcess,               // ESRCH
    TimedOut,                // ETIMEDOUT
    NotFound,                // ENOTFOUND
    NoFilesystem,            // ENOFS
    ArgumentSecurity,        // EARGSEC
    InvalidSession,          // EINVSESSION
}

pub type Result<T> = core::result::Result<T, Status>;

impl Status {
    pub fn to_return_code(self) -> isize {
        -(self as isize)
    }
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Status::Success => "Success",
            Status::OutOfDomain => "Out of domain",
            Status::OutOfRange => "Out of range",
            Status::PermissionDenied => "Permission denied",
            Status::InUse => "In use",
            Status::NotAvailable => "Not available",
            Status::NotSupported => "Not supported",
            Status::TryAgain => "Try again",
            Status::AlreadyInProgress => "Already in progress",
            Status::BadDescriptor => "Bad descriptor",
            Status::BadMessage => "Bad message",
            Status::InvalidRequestCode => "Invalid request code",
            Status::Busy => "Busy",
            Status::Cancelled => "Cancelled",
            Status::Exists => "Exists",
            Status::BadAddress => "Bad address",
            Status::TooBig => "Too big",
            Status::InvalidUTF8 => "Invalid UTF8",
            Status::InProgress => "In progress",
            Status::InvalidArgument => "Invalid argument",
            Status::IOError => "I/O error",
            Status::IsDirectory => "Is directory",
            Status::NameTooLong => "Name too long",
            Status::NoDevice => "No device",
            Status::NoEntry => "No entry",
            Status::InvalidExecutableFormat => "Invalid executable format",
            Status::NoSpace => "No space",
            Status::NotImplemented => "Not implemented",
            Status::NotDirectory => "Not directory",
            Status::NotEmpty => "Not empty",
            Status::InvalidIOCtrl => "Invalid I/O control",
            Status::NotUnique => "Not unique",
            Status::ReadOnlyFilesystem => "Read-only filesystem",
            Status::NoProcess => "No process",
            Status::TimedOut => "Timed-out",
            Status::NotFound => "Not found",
            Status::NoFilesystem => "No filesystem",
            Status::ArgumentSecurity => "Argument security error",
            Status::InvalidSession => "Invalid session",
        })
    }
}
