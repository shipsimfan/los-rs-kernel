use super::NamePath;
use crate::aml::{ByteStream, Result};

enum PathClass {
    Root,
    Prefix(usize),
}

pub(in crate::aml) struct NameString {
    class: PathClass,
    path: NamePath,
}

fn parse_prefix(stream: &mut ByteStream) -> usize {
    let mut prefix = 0;
    while let Some(c) = stream.peek() {
        if c == b'^' {
            prefix += 1;
            stream.next();
        } else {
            break;
        }
    }
    prefix
}

impl NameString {
    pub(in crate::aml) fn parse(stream: &mut ByteStream) -> Result<Self> {
        let class = match stream.peek() {
            Some(c) => match c {
                b'^' => PathClass::Prefix(parse_prefix(stream)),
                b'\\' => {
                    stream.next();
                    PathClass::Root
                }
                _ => PathClass::Prefix(0),
            },
            None => return Err(crate::aml::Error::UnexpectedEndOfStream),
        };

        let path = NamePath::parse(stream)?;

        Ok(NameString { class, path })
    }
}

impl core::fmt::Display for NameString {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}{}", self.class, self.path)
    }
}

impl core::fmt::Display for PathClass {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            PathClass::Prefix(count) => {
                for _ in 0..*count {
                    write!(f, "^")?;
                }
                Ok(())
            }
            PathClass::Root => write!(f, "\\"),
        }
    }
}
