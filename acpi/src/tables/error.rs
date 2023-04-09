use crate::parser;

pub(super) type Result<T> = core::result::Result<T, Error>;

enum ErrorKind {
    InvalidTable,
    MissingTable,
    Parser(parser::Error),
}

pub(crate) struct Error {
    table: &'static [u8],
    kind: ErrorKind,
}

impl Error {
    pub(super) fn invalid_table(table: &'static [u8]) -> Self {
        Error {
            table,
            kind: ErrorKind::InvalidTable,
        }
    }

    pub(super) fn missing_table(table: &'static [u8]) -> Self {
        Error {
            table,
            kind: ErrorKind::MissingTable,
        }
    }

    pub(super) fn parser_error(table: &'static [u8], error: parser::Error) -> Self {
        Error {
            table,
            kind: ErrorKind::Parser(error),
        }
    }
}

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Error while loading ")?;
        for byte in self.table {
            write!(f, "{}", *byte as char)?;
        }
        write!(f, " - {}", self.kind)
    }
}

impl core::fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            ErrorKind::InvalidTable => write!(f, "Invalid Table"),
            ErrorKind::MissingTable => write!(f, "Missing Table"),
            ErrorKind::Parser(error) => error.fmt(f),
        }
    }
}
