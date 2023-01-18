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

impl From<ErrorKind> for Error {
    fn from(kind: ErrorKind) -> Self {
        Error::new(kind)
    }
}
