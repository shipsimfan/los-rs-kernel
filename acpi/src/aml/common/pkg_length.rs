use crate::aml::{next, ByteStream, Result};

pub(in crate::aml) struct PkgLength;

impl PkgLength {
    pub(in crate::aml) fn parse(stream: &mut ByteStream) -> Result<usize> {
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
}
