pub(super) type Result<T> = core::result::Result<T, Error>;

enum ErrorKind {
    UnexpectedEndOfStream,
    UnexpectedByte(u8),
}

pub(crate) struct Error {
    kind: ErrorKind,
    offset: usize,
}

impl Error {
    pub(super) fn unexpected_end_of_stream(offset: usize) -> Self {
        Self::new(ErrorKind::UnexpectedEndOfStream, offset)
    }

    pub(super) fn unexpected_byte(byte: u8, offset: usize) -> Self {
        Self::new(ErrorKind::UnexpectedByte(byte), offset)
    }

    fn new(kind: ErrorKind, offset: usize) -> Self {
        Error { kind, offset }
    }
}

impl core::fmt::Debug for Error {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Display::fmt(self, f)
    }
}

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "Unable to parse AML - {} at offset {}",
            self.kind, self.offset
        )
    }
}

impl core::fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            ErrorKind::UnexpectedEndOfStream => write!(f, "Unexpected end of stream"),
            ErrorKind::UnexpectedByte(byte) => write!(f, "Unexpected byte {:#04X}", byte),
        }
    }
}
