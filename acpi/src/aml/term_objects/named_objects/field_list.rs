use super::FieldElement;
use crate::aml::{impl_core_display, Display, Result, Stream};
use alloc::vec::Vec;

pub(in crate::aml) struct FieldList {
    list: Vec<FieldElement>,
}

impl FieldList {
    pub(in crate::aml) fn parse(_stream: &mut Stream) -> Result<Self> {
        return Ok(FieldList { list: Vec::new() });

        /*
        TODO: Figure out how to parse fields

        let mut list = Vec::new();
        while stream.peek().is_some() {
            list.push(FieldElement::parse(stream)?);
        }

        Ok(FieldList { list })
        */
    }
}

impl Display for FieldList {
    fn display(&self, f: &mut core::fmt::Formatter, depth: usize, last: bool) -> core::fmt::Result {
        if self.list.len() == 0 {
            return writeln!(f, "{{}}");
        }

        writeln!(f, "{{")?;

        for field_element in &self.list {
            field_element.display(f, depth + 1, last)?;
        }

        self.display_prefix(f, depth)?;
        writeln!(f, "}}")
    }
}

impl_core_display!(FieldList);
