use crate::parser;

pub(crate) enum Error {
    Parse(parser::Error),
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
        }
    }
}

impl From<parser::Error> for Error {
    fn from(error: parser::Error) -> Self {
        Error::Parse(error)
    }
}
