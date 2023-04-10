use super::{next, Result, Stream};

pub(super) fn parse(stream: &mut Stream, source: &'static str) -> Result<usize> {
    let first = next!(stream, source);

    let byte_count = first.wrapping_shr(6);
    if byte_count == 0 {
        Ok((first & 0x3F) as usize - 1)
    } else {
        let mut length = (first & 0xF) as usize;
        let mut shift = 4;
        for _ in 0..byte_count {
            let c = next!(stream, source);
            length |= (c as usize) << shift;
            shift += 8;
        }
        Ok(length - byte_count as usize - 1)
    }
}

pub(super) fn parse_to_stream<'a>(
    stream: &mut Stream<'a>,
    source: &'static str,
) -> Result<Stream<'a>> {
    let amount = parse(stream, source)?;
    stream.collect_to_stream(amount, source)
}
