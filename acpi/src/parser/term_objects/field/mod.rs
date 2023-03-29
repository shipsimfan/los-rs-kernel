use crate::parser::{pkg_length, NameString, Result, Stream};

mod flags;

pub(crate) use flags::*;

pub(crate) struct Field<'a> {
    name: NameString,
    flags: FieldFlags,
    field_units: &'a [u8],
}

impl<'a> Field<'a> {
    pub(super) fn parse(stream: &mut Stream<'a>) -> Result<Self> {
        let mut stream = pkg_length::parse_to_stream(stream)?;

        let name = NameString::parse(&mut stream)?;
        let flags = FieldFlags::parse(&mut stream)?;
        let field_units = stream.collect_bytes(stream.remaining())?;

        Ok(Field {
            name,
            flags,
            field_units,
        })
    }

    pub(crate) fn name(&self) -> &NameString {
        &self.name
    }

    pub(crate) fn flags(&self) -> FieldFlags {
        self.flags
    }

    pub(crate) fn field_units(&self) -> &'a [u8] {
        self.field_units
    }
}
