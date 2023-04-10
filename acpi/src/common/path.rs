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

    pub(crate) fn join(&self, other: &Path) -> Self {
        let super_count = match other.prefix {
            PathPrefix::Root => return other.clone(),
            PathPrefix::Super(count) => count,
            PathPrefix::None => 0,
        };

        let mut new_path = self.clone();

        if let Some(r#final) = new_path.r#final.take() {
            new_path.path.push(r#final);
        }

        for _ in 0..super_count {
            new_path.path.pop();
        }

        new_path.path.extend_from_slice(&other.path);
        new_path.r#final = other.r#final.clone();

        new_path
    }
}

impl<'a> TryFrom<&'a str> for Path {
    type Error = InvalidPathError;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        let source = value.as_bytes();

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

            let name = Name::try_from(&source[offset..offset + len])?;

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

impl PartialEq for Path {
    fn eq(&self, other: &Self) -> bool {
        assert!(self.prefix == PathPrefix::Root && other.prefix == PathPrefix::Root);

        if self.path.len() != other.path.len() {
            return false;
        }

        for i in 0..self.path.len() {
            if self.path[i] != other.path[i] {
                return false;
            }
        }

        self.r#final == other.r#final
    }
}

impl Eq for Path {}

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
