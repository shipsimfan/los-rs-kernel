use crate::aml::{next, Result, Stream};

pub(super) fn parse(stream: &mut Stream) -> Result<usize> {
    let first = next!(stream);

    let byte_count = first.wrapping_shr(6);
    if byte_count == 0 {
        Ok((first & 0x3F) as usize - 1)
    } else {
        let mut length = (first & 0xF) as usize;
        let mut shift = 4;
        for _ in 0..byte_count {
            let c = next!(stream);
            length |= (c as usize) << shift;
            shift += 8;
        }
        Ok(length - byte_count as usize - 1)
    }
}

pub(super) fn parse_to_stream<'a>(stream: &mut Stream<'a>) -> Result<Stream<'a>> {
    let amount = parse(stream)?;
    stream.collect_to_stream(amount)
}
