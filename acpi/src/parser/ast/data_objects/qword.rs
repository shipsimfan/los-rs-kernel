use crate::parser::{next, Result, Stream};

pub(super) fn parse(stream: &mut Stream) -> Result<u64> {
    Ok(u64::from_le_bytes([
        next!(stream, "QWord"),
        next!(stream, "QWord"),
        next!(stream, "QWord"),
        next!(stream, "QWord"),
        next!(stream, "QWord"),
        next!(stream, "QWord"),
        next!(stream, "QWord"),
        next!(stream, "QWord"),
    ]))
}
