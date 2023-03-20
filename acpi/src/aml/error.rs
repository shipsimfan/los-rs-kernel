pub(crate) enum Error {
    UnexpectedByte(u8),
    UnexpectedEndOfStream,
}

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Error::UnexpectedByte(byte) => write!(f, "Unexpected byte {:#X}", byte),
            Error::UnexpectedEndOfStream => write!(f, "Unexpected end of stream"),
        }
    }
}
