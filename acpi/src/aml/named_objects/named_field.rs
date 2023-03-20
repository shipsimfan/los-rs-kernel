use crate::aml::{ByteStream, NameSeg, PkgLength, Result};

pub(super) struct NamedField {
    name: NameSeg,
    length: usize,
}

impl NamedField {
    pub(super) fn parse(stream: &mut ByteStream) -> Result<Self> {
        let name = NameSeg::parse(stream)?;
        let length = PkgLength::parse(stream)?;

        Ok(NamedField { name, length })
    }
}
