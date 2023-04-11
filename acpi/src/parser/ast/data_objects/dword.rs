use crate::parser::{next, Result, Stream};

pub(super) fn parse(stream: &mut Stream) -> Result<u32> {
    Ok(u32::from_le_bytes([
        next!(stream, "DWord"),
        next!(stream, "DWord"),
        next!(stream, "DWord"),
        next!(stream, "DWord"),
    ]))
}
