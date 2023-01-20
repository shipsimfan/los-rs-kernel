use crate::ErrorKind;
use alloc::string::String;

pub struct Error {
    kind: ErrorKind,
    message: Option<String>,
}

impl Error {
    pub fn new(kind: ErrorKind) -> Self {
        Error {
            kind,
            message: None,
        }
    }

    pub fn new_message(kind: ErrorKind, message: String) -> Self {
        Error {
            kind,
            message: Some(message),
        }
    }

    pub fn kind(&self) -> ErrorKind {
        self.kind
    }

    pub fn message(&self) -> Option<&str> {
        self.message.as_ref().map(|message| message.as_str())
    }
}

impl core::fmt::Debug for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Display::fmt(self, f)
    }
}

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self.message.as_ref() {
            Some(message) => write!(f, "{}", message),
            None => write!(f, "{}", self.kind),
        }
    }
}

impl From<ErrorKind> for Error {
    fn from(kind: ErrorKind) -> Self {
        Error::new(kind)
    }
}
