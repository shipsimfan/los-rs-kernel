use crate::parser::{next, Result, Stream};

pub(super) fn parse(stream: &mut Stream) -> Result<u8> {
    Ok(next!(stream, "Byte"))
}
