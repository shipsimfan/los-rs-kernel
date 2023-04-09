use super::{InvalidNameError, Name};
use alloc::vec::Vec;

#[derive(Clone, PartialEq, Eq)]
pub(crate) enum PathPrefix {
    None,
    Super(usize),
    Root,
}

#[derive(Clone)]
pub(crate) struct Path {
    prefix: PathPrefix,
    path: Vec<Name>,
    r#final: Option<Name>,
}

pub(crate) struct InvalidPathError;

impl Path {
    pub(crate) fn new(prefix: PathPrefix, path: Vec<Name>, r#final: Option<Name>) -> Self {
        Path {
            prefix,
            path,
            r#final,
        }
    }

    pub(crate) fn parse(source: &[u8]) -> Result<Self, InvalidPathError> {
        let (prefix, mut offset) = PathPrefix::parse(source);

        let mut path = Vec::new();
        loop {
            if offset == source.len() {
                return Ok(Path::new(prefix, path, None));
            }

            let mut len = 0;
            while len < 4
                && offset + len < source.len()
                && source[offset + len] != b'/'
                && source[offset + len] != b'\\'
            {
                len += 1;
            }

            let name = Name::parse(&source[offset..offset + len])?;

            if offset + len == source.len() {
                return Ok(Path::new(prefix, path, Some(name)));
            } else if source[offset + len] == b'/' || source[offset + len] == b'\\' {
                path.push(name);
                offset += len + 1;
            } else {
                return Err(InvalidPathError);
            }
        }
    }
}

impl<'a> TryFrom<&'a str> for Path {
    type Error = InvalidPathError;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        Path::parse(value.as_bytes())
    }
}

impl core::fmt::Display for Path {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.prefix)?;

        for part in &self.path {
            write!(f, "{}", part)?;
            write!(f, "/")?;
        }

        match &self.r#final {
            Some(r#final) => write!(f, "{}", r#final),
            None => Ok(()),
        }
    }
}

impl PathPrefix {
    pub(self) fn parse(prefix: &[u8]) -> (Self, usize) {
        if prefix.len() == 0 {
            return (PathPrefix::None, 0);
        }

        match prefix[0] {
            b'/' => return (PathPrefix::Root, 1),
            b'^' => {}
            _ => return (PathPrefix::None, 0),
        }

        let mut i = 1;
        while i < prefix.len() && prefix[i] == b'^' {
            i += 1;
        }

        (PathPrefix::Super(i), i)
    }
}

impl core::fmt::Display for PathPrefix {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            PathPrefix::None => Ok(()),
            PathPrefix::Root => write!(f, "/"),
            PathPrefix::Super(count) => {
                for _ in 0..*count {
                    write!(f, "^")?;
                }

                Ok(())
            }
        }
    }
}

impl core::fmt::Debug for InvalidPathError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Display::fmt(self, f)
    }
}

impl core::fmt::Display for InvalidPathError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Invalid ACPI path")
    }
}

impl From<InvalidNameError> for InvalidPathError {
    fn from(_: InvalidNameError) -> Self {
        InvalidPathError
    }
}
