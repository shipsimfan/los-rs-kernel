use super::NameSeg;
use crate::aml::{ByteStream, Result};

pub(super) enum NamePath {
    Null,
    NameSeg(NameSeg),
}

impl NamePath {
    pub(super) fn parse(stream: &mut ByteStream) -> Result<Self> {
        let c = match stream.peek() {
            Some(c) => c,
            None => return Err(crate::aml::Error::UnexpectedEndOfStream),
        };

        if c == 0 {
            stream.next();
            Ok(NamePath::Null)
        } else {
            Ok(NamePath::NameSeg(NameSeg::parse(stream)?))
        }
    }
}

impl core::fmt::Display for NamePath {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            NamePath::Null => Ok(()),
            NamePath::NameSeg(name) => write!(f, "{}", name),
        }
    }
}
