use crate::parser::{self, NameString};

pub(crate) enum Error {
    Parse(parser::Error),
    UnknownName(NameString),
    InvalidParent(NameString),
    InvalidName(NameString),
    InvalidType(NameString),
    NameCollision(NameString),
}

pub(super) type Result<T> = core::result::Result<T, Error>;

impl core::fmt::Debug for Error {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Display::fmt(self, f)
    }
}

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Error::Parse(error) => error.fmt(f),
            Error::UnknownName(name) => write!(f, "Unknown name \"{}\"", name),
            Error::InvalidParent(name) => write!(f, "Invalid parent \"{}\"", name),
            Error::InvalidName(name) => write!(f, "Invalid name \"{}\"", name),
            Error::InvalidType(name) => write!(f, "Invalid type around \"{}\"", name),
            Error::NameCollision(name) => write!(f, "Name collision \"{}\"", name),
        }
    }
}

impl From<parser::Error> for Error {
    fn from(error: parser::Error) -> Self {
        Error::Parse(error)
    }
}
