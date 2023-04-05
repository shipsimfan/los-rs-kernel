use super::TermList;
use crate::parser::{next, pkg_length, NameString, Result, Stream};

pub(crate) struct ThermalZone<'a> {
    name: NameString,
    term_list: TermList<'a>,
}

impl<'a> ThermalZone<'a> {
    pub(super) fn parse(stream: &mut Stream<'a>) -> Result<Self> {
        let mut stream = pkg_length::parse_to_stream(stream)?;

        let name = NameString::parse(&mut stream)?;
        let term_list = TermList::parse(stream);

        Ok(ThermalZone { name, term_list })
    }

    pub(crate) fn name(&self) -> &NameString {
        &self.name
    }

    pub(crate) fn term_list(&mut self) -> &mut TermList<'a> {
        &mut self.term_list
    }
}