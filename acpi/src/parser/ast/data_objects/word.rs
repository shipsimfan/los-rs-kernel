use crate::parser::{next, Result, Stream};

pub(super) fn parse(stream: &mut Stream) -> Result<u16> {
    Ok(u16::from_le_bytes([
        next!(stream, "Word"),
        next!(stream, "Word"),
    ]))
}
