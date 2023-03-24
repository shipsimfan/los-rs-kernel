use super::{FieldFlags, FieldList};
use crate::aml::{impl_core_display, pkg_length, Display, NameString, Result, Stream};

pub(in crate::aml::term_objects) struct IndexField {
    index_name: NameString,
    data_name: NameString,
    flags: FieldFlags,
    field_list: FieldList,
}

impl IndexField {
    pub(super) fn parse(stream: &mut Stream) -> Result<Self> {
        let mut stream = pkg_length::parse_to_stream(stream)?;

        let index_name = NameString::parse(&mut stream)?;
        let data_name = NameString::parse(&mut stream)?;
        let flags = FieldFlags::parse(&mut stream)?;
        let field_list = FieldList::parse(&mut stream)?;

        Ok(IndexField {
            index_name,
            data_name,
            flags,
            field_list,
        })
    }
}

impl Display for IndexField {
    fn display(&self, f: &mut core::fmt::Formatter, depth: usize, last: bool) -> core::fmt::Result {
        self.display_prefix(f, depth)?;
        write!(
            f,
            "Index Field ({}, {}, {}) ",
            self.index_name, self.data_name, self.flags
        )?;

        self.field_list.display(f, depth, last)
    }
}

impl_core_display!(IndexField);
