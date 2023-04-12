use crate::{
    parser::{next, Error, Result, Stream},
    Name, Path, PathPrefix,
};
use alloc::vec::Vec;

pub(in crate::parser) fn parse(stream: &mut Stream, source: &'static str) -> Result<Path> {
    let prefix = parse_prefix(stream, source)?;

    let offset = stream.offset();
    let c = next!(stream, source);
    let (count, c) = if c.is_ascii_uppercase() || c == b'_' {
        (1, Some(c))
    } else if c == 0x2E {
        (2, None)
    } else if c == 0x2F {
        (next!(stream, source) as usize, None)
    } else if c == 0x00 {
        return Ok(Path::new(PathPrefix::None, Vec::new(), None));
    } else {
        return Err(Error::unexpected_byte(c, offset, source));
    };

    let mut path = Vec::with_capacity(count - 1);
    for _ in 0..count - 1 {
        path.push(parse_name_seg(stream, None, source)?);
    }

    let name = parse_name_seg(stream, c, source)?;

    Ok(Path::new(prefix, path, Some(name)))
}

fn parse_prefix(stream: &mut Stream, source: &'static str) -> Result<PathPrefix> {
    Ok(match next!(stream, source) {
        b'^' => {
            let mut count = 1;
            while let Some(c) = stream.next() {
                if c == b'^' {
                    stream.next();
                    count += 1;
                } else {
                    stream.prev();
                    break;
                }
            }
            PathPrefix::Super(count)
        }
        b'\\' => PathPrefix::Root,
        _ => {
            stream.prev();
            PathPrefix::None
        }
    })
}

fn parse_name_seg(stream: &mut Stream, c: Option<u8>, source: &'static str) -> Result<Name> {
    let (mut name, start) = match c {
        Some(c) => ([c, 0, 0, 0], 1),
        None => ([0; 4], 0),
    };

    let offset = stream.offset() - start;

    for i in start..4 {
        name[i] = next!(stream, source);
    }

    Name::new(name).map_err(|error| Error::invalid_name(error, offset, source))
}
