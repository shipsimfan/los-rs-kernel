use crate::{InvalidNameError, Path};

pub(super) type Result<T> = core::result::Result<T, Error>;

pub(crate) enum ErrorKind {
    UnexpectedByte(u8),
    UnexpectedEndOfStream,
    InvalidName(InvalidNameError),
    NameCollision(Path),
}

pub(crate) struct Error {
    offset: usize,
    source: &'static str,
    kind: ErrorKind,
}

impl Error {
    pub(super) fn unexpected_byte(byte: u8, offset: usize, source: &'static str) -> Self {
        Error {
            offset,
            source,
            kind: ErrorKind::UnexpectedByte(byte),
        }
    }

    pub(super) fn unexpected_end_of_stream(offset: usize, source: &'static str) -> Self {
        Error {
            offset,
            source,
            kind: ErrorKind::UnexpectedEndOfStream,
        }
    }

    pub(super) fn invalid_name(
        error: InvalidNameError,
        offset: usize,
        source: &'static str,
    ) -> Self {
        Error {
            offset,
            source,
            kind: ErrorKind::InvalidName(error),
        }
    }

    pub(super) fn name_collision(path: Path, offset: usize, source: &'static str) -> Self {
        Error {
            offset,
            source,
            kind: ErrorKind::NameCollision(path),
        }
    }
}

impl core::fmt::Debug for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Display::fmt(self, f)
    }
}

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "{} for {} at offset {}",
            self.kind, self.source, self.offset
        )
    }
}

impl core::fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            ErrorKind::UnexpectedByte(byte) => write!(f, "Unexpected byte {:#04X}", *byte),
            ErrorKind::UnexpectedEndOfStream => write!(f, "Unexpected end of stream"),
            ErrorKind::InvalidName(invalid_name) => invalid_name.fmt(f),
            ErrorKind::NameCollision(path) => {
                write!(f, "Two methods defined with the same name ({})", path)
            }
        }
    }
}
