use super::{FieldFlags, FieldList};
use crate::aml::{impl_core_display, pkg_length, Display, NameString, Result, Stream};

pub(in crate::aml::term_objects) struct Field {
    offset: usize,
    name: NameString,
    flags: FieldFlags,
    field_list: FieldList,
}

impl Field {
    pub(super) fn parse(stream: &mut Stream) -> Result<Self> {
        let offset = stream.offset() - 2;

        let mut stream = pkg_length::parse_to_stream(stream)?;

        let name = NameString::parse(&mut stream)?;
        let flags = FieldFlags::parse(&mut stream)?;
        let field_list = FieldList::parse(&mut stream)?;

        Ok(Field {
            offset,
            name,
            flags,
            field_list,
        })
    }
}

impl Display for Field {
    fn display(&self, f: &mut core::fmt::Formatter, depth: usize) -> core::fmt::Result {
        self.display_prefix(f, depth)?;
        writeln!(f, "Field {} ({}) @ {}", self.name, self.flags, self.offset)?;

        self.field_list.display(f, depth + 1)
    }
}

impl_core_display!(Field);
