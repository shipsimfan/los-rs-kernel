use super::{FieldFlags, FieldList};
use crate::aml::{impl_core_display, pkg_length, Display, NameString, Result, Stream};

pub(in crate::aml::term_objects) struct IndexField {
    offset: usize,
    name1: NameString,
    name2: NameString,
    flags: FieldFlags,
    field_list: FieldList,
}

impl IndexField {
    pub(super) fn parse(stream: &mut Stream) -> Result<Self> {
        let offset = stream.offset() - 2;

        let mut stream = pkg_length::parse_to_stream(stream)?;

        let name1 = NameString::parse(&mut stream)?;
        let name2 = NameString::parse(&mut stream)?;
        let flags = FieldFlags::parse(&mut stream)?;
        let field_list = FieldList::parse(&mut stream)?;

        Ok(IndexField {
            offset,
            name1,
            name2,
            flags,
            field_list,
        })
    }
}

impl Display for IndexField {
    fn display(&self, f: &mut core::fmt::Formatter, depth: usize) -> core::fmt::Result {
        self.display_prefix(f, depth)?;
        writeln!(
            f,
            "Index Field {} -> {} ({}) @ {}",
            self.name1, self.name2, self.flags, self.offset
        )?;

        self.field_list.display(f, depth + 1)
    }
}

impl_core_display!(IndexField);
