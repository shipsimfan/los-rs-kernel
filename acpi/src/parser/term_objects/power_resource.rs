use crate::parser::{next, pkg_length, NameString, Result, Stream};

use super::TermList;

pub(crate) struct PowerResource<'a> {
    name: NameString,
    system_level: u8,
    resource_order: u16,
    term_list: TermList<'a>,
}

impl<'a> PowerResource<'a> {
    pub(super) fn parse(stream: &mut Stream<'a>) -> Result<Self> {
        let mut stream = pkg_length::parse_to_stream(stream)?;

        let name = NameString::parse(&mut stream)?;
        let system_level = next!(stream);
        let resource_order = u16::from_le_bytes([next!(stream), next!(stream)]);
        let term_list = TermList::parse(stream);

        Ok(PowerResource {
            name,
            system_level,
            resource_order,
            term_list,
        })
    }

    pub(crate) fn name(&self) -> &NameString {
        &self.name
    }

    pub(crate) fn system_level(&self) -> u8 {
        self.system_level
    }

    pub(crate) fn resource_order(&self) -> u16 {
        self.resource_order
    }

    pub(crate) fn term_list(&mut self) -> &mut TermList<'a> {
        &mut self.term_list
    }
}
