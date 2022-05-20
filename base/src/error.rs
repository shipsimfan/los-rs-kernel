use alloc::boxed::Box;
use core::fmt::{Debug, Display};

pub type Result<T> = core::result::Result<T, Box<dyn Error>>;

pub const DEVICE_MODULE_NUMBER: i32 = 0;
pub const SESSION_MODULE_NUMBER: i32 = 1;
pub const TIME_MODULE_NUMBER: i32 = 2;
pub const FILESYSTEM_MODULE_NUMBER: i32 = 3;

pub const UEFI_DRIVER_MODULE_NUMBER: i32 = 0x1000000;
pub const HPET_DRIVER_MODULE_NUMBER: i32 = 0x1000001;
pub const PCI_DRIVER_MODULE_NUMBER: i32 = 0x1000002;
pub const IDE_DRIVER_MODULE_NUMBER: i32 = 0x1000002;

#[derive(Debug, PartialEq, Eq)]
#[repr(u32)]
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
    ReadOnly,                // ERDONLY
    WriteOnly,               // EWRONLY
    OutOfResource,           // ENORESOURCE
    CorruptFilesystem,       // ECORRUPTFILESYSTEM
    IsFile,                  // EISFILE
    InvalidThread,           // EINVTHREAD
    Interrupted,             // EINT
    InvalidPath,             // EPATH
}

pub trait Error: Debug + Display {
    fn module_number(&self) -> i32;
    fn error_number(&self) -> Status;

    #[inline(always)]
    fn to_status_code(&self) -> isize {
        let module_number = self.module_number();
        assert!(module_number >= 0);

        -((((module_number as usize) << ((core::mem::size_of::<usize>() / 2) * 8))
            + self.error_number() as usize) as isize)
    }
}
