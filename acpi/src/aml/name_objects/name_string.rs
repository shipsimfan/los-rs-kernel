use crate::aml::{next, peek, Error, Result, Stream};
use alloc::vec::Vec;

pub(in crate::aml) enum Prefix {
    None,
    Super(usize),
    Root,
}

pub(in crate::aml) fn parse(
    stream: &mut Stream,
) -> Result<(Prefix, Vec<[u8; 4]>, Option<[u8; 4]>)> {
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
        return Ok((prefix, Vec::new(), None));
    } else {
        return Err(Error::UnexpectedByte(offset, c));
    };

    let mut path = Vec::with_capacity(count - 1);
    for _ in 0..count - 1 {
        path.push(parse_name_seg(stream, None)?);
    }

    let name = parse_name_seg(stream, c)?;

    Ok((prefix, path, Some(name)))
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

fn parse_name_seg(stream: &mut Stream, c: Option<u8>) -> Result<[u8; 4]> {
    let (mut name, start) = match c {
        Some(c) => ([c, 0, 0, 0], 1),
        None => ([0; 4], 0),
    };

    for i in start..4 {
        name[i] = next!(stream);
    }

    Ok(name)
}
