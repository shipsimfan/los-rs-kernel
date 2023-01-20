#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ErrorKind {
    Interrupted,
    InvalidArgument,
}

impl core::fmt::Debug for ErrorKind {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Display::fmt(self, f)
    }
}

impl core::fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ErrorKind::Interrupted => "Operation interrupted",
                ErrorKind::InvalidArgument => "Invalid argument",
            }
        )
    }
}
