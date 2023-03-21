pub(crate) enum Error {
    UnexpectedEndOfStream(usize),
    UnexpectedByte(usize, u8),
    MissingName(Option<[u8; 4]>),
    NameCollision([u8; 4]),
    AddChildNotScope,
    InvalidArgumentType,
}

impl Error {
    pub(super) fn unexpected_end_of_stream(offset: usize) -> Self {
        Error::UnexpectedEndOfStream(offset)
    }

    pub(super) fn unexpected_byte(offset: usize, byte: u8) -> Self {
        Error::UnexpectedByte(offset, byte)
    }

    pub(super) fn missing_name(name: Option<[u8; 4]>) -> Self {
        Error::MissingName(name)
    }
}

impl core::fmt::Debug for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Display::fmt(self, f)
    }
}

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Error while parsing AML - ")?;

        match self {
            Error::UnexpectedEndOfStream(offset) => {
                write!(f, "Unexpected end of stream at offset {}", offset)
            }
            Error::UnexpectedByte(offset, byte) => {
                write!(f, "Unexpected byte {:#X} at offset {}", byte, offset)
            }
            Error::MissingName(name) => {
                write!(f, "Missing name",)?;
                match name {
                    Some(name) => write!(
                        f,
                        " \"{}{}{}{}\"",
                        name[0] as char, name[1] as char, name[2] as char, name[3] as char
                    ),
                    None => Ok(()),
                }
            }
            Error::NameCollision(name) => write!(
                f,
                "Redefinition of \"{}{}{}{}\"",
                name[0] as char, name[1] as char, name[2] as char, name[3] as char
            ),
            Error::AddChildNotScope => {
                write!(f, "Attempting to add a child to an object without a scope")
            }
            Error::InvalidArgumentType => write!(f, "Invalid argument type"),
        }
    }
}
