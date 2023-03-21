use crate::aml;

enum ErrorKind {
    InvalidTable,
    MissingTable,
    AMLError(aml::Error),
}

pub(crate) struct Error {
    table: &'static [u8],
    kind: ErrorKind,
}

impl Error {
    pub(crate) fn invalid_table(table: &'static [u8]) -> Self {
        Error {
            table,
            kind: ErrorKind::InvalidTable,
        }
    }

    pub(crate) fn missing_table(table: &'static [u8]) -> Self {
        Error {
            table,
            kind: ErrorKind::MissingTable,
        }
    }

    pub(super) fn aml_error(table: &'static [u8], error: aml::Error) -> Self {
        Error {
            table,
            kind: ErrorKind::AMLError(error),
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
            ErrorKind::AMLError(error) => error.fmt(f),
        }
    }
}
