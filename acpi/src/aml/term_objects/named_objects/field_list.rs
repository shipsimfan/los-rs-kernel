use super::FieldElement;
use crate::aml::{impl_core_display, Display, Result, Stream};
use alloc::vec::Vec;

pub(in crate::aml) struct FieldList {
    offset: usize,
    list: Vec<FieldElement>,
}

impl FieldList {
    pub(in crate::aml) fn parse(stream: &mut Stream) -> Result<Self> {
        let offset = stream.offset();

        return Ok(FieldList {
            offset,
            list: Vec::new(),
        });

        /*
        TODO: Figure out how to parse fields


        let mut list = Vec::new();
        while stream.peek().is_some() {
            list.push(FieldElement::parse(stream)?);
        }

        Ok(FieldList { offset, list })
        */
    }
}

impl Display for FieldList {
    fn display(&self, f: &mut core::fmt::Formatter, depth: usize) -> core::fmt::Result {
        self.display_prefix(f, depth)?;
        writeln!(f, "Field List @ {}:", self.offset)?;

        for field_element in &self.list {
            field_element.display(f, depth + 1)?;
        }

        Ok(())
    }
}

impl_core_display!(FieldList);
