use super::{FieldFlags, FieldList};
use crate::aml::{ASTNode, ByteStream, NameString, PkgLength, Result};

pub(in crate::aml) struct DefField {
    name: NameString,
    flags: FieldFlags,
    list: FieldList,
}

impl DefField {
    pub(super) fn parse(stream: &mut ByteStream) -> Result<Self> {
        let length = PkgLength::parse(stream)?;
        let mut stream = ByteStream::new(stream.collect(length)?);

        let name = NameString::parse(&mut stream)?;
        let flags = FieldFlags::parse(&mut stream)?;
        let list = FieldList::parse(&mut stream)?;

        Ok(DefField { name, flags, list })
    }
}

impl ASTNode for DefField {
    fn display(&self, f: &mut core::fmt::Formatter, depth: usize) -> core::fmt::Result {
        todo!()
    }
}
