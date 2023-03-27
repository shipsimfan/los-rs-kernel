use crate::aml::{next, peek, Error, Result, Stream};
use alloc::vec::Vec;

enum Prefix {
    None,
    Super(usize),
    Root,
}

pub(super) struct NameString {
    prefix: Prefix,
    path: Vec<[u8; 4]>,
    name: Option<[u8; 4]>,
}

fn parse_prefix(stream: &mut Stream) -> Result<Prefix> {
    Ok(match peek!(stream) {
        b'^' => {
            let mut count = 0;
            while let Some(c) = stream.peek() {
                if c == b'^' {
                    stream.next();
                    count += 1;
                } else {
                    break;
                }
            }
            Prefix::Super(count)
        }
        b'\\' => {
            stream.next();
            Prefix::Root
        }
        _ => Prefix::None,
    })
}

pub(super) fn parse_name_seg(stream: &mut Stream, c: Option<u8>) -> Result<[u8; 4]> {
    let (mut name, start) = match c {
        Some(c) => ([c, 0, 0, 0], 1),
        None => ([0; 4], 0),
    };

    for i in start..4 {
        name[i] = next!(stream);
    }

    Ok(name)
}

impl NameString {
    pub(super) fn parse(stream: &mut Stream) -> Result<Self> {
        let prefix = parse_prefix(stream)?;

        let offset = stream.offset();
        let c = next!(stream);
        let (count, c) = if c.is_ascii_uppercase() || c == b'_' {
            (1, Some(c))
        } else if c == 0x2E {
            (2, None)
        } else if c == 0x2F {
            (next!(stream) as usize, None)
        } else if c == 0x00 {
            return Ok(NameString {
                prefix: Prefix::None,
                path: Vec::new(),
                name: None,
            });
        } else {
            return Err(Error::unexpected_byte(c, offset)).unwrap();
        };

        let mut path = Vec::with_capacity(count - 1);
        for _ in 0..count - 1 {
            path.push(parse_name_seg(stream, None)?);
        }

        let name = parse_name_seg(stream, c)?;

        Ok(NameString {
            prefix,
            path,
            name: Some(name),
        })
    }
}

impl core::fmt::Display for NameString {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.prefix)?;

        for part in &self.path {
            write!(
                f,
                "{}{}{}{}\\",
                part[0] as char, part[1] as char, part[2] as char, part[3] as char
            )?;
        }

        match self.name {
            Some(name) => write!(
                f,
                "{}{}{}{}",
                name[0] as char, name[1] as char, name[2] as char, name[3] as char
            ),
            None => Ok(()),
        }
    }
}

impl core::fmt::Display for Prefix {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Prefix::None => Ok(()),
            Prefix::Root => write!(f, "\\"),
            Prefix::Super(count) => {
                for _ in 0..*count {
                    write!(f, "^")?;
                }
                Ok(())
            }
        }
    }
}
