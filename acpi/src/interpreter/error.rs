use crate::{
    namespace::display_name,
    parser::{self, NameString},
};

pub(crate) enum Error {
    Parse(parser::Error),
    UnknownName(NameString),
    InvalidParent(NameString),
    InvalidName(NameString),
    InvalidType(NameString),
    NameCollision(NameString),

    InvalidArgumentCount([u8; 4]),
    InvalidNodeType([u8; 4]),
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

            Error::InvalidArgumentCount(name) => {
                write!(f, "Invalid argument count for \"")?;
                display_name!(f, *name);
                write!(f, "\"")
            }
            Error::InvalidNodeType(name) => {
                write!(f, "Invalid node type \"")?;
                display_name!(f, *name);
                write!(f, "\"")
            }
        }
    }
}

impl From<parser::Error> for Error {
    fn from(error: parser::Error) -> Self {
        Error::Parse(error)
    }
}
