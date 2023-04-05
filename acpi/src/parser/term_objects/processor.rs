use super::TermList;
use crate::parser::{next, pkg_length, NameString, Result, Stream};

pub(crate) struct Processor<'a> {
    name: NameString,
    id: u8,
    address: u32,
    length: u8,
    term_list: TermList<'a>,
}

impl<'a> Processor<'a> {
    pub(super) fn parse(stream: &mut Stream<'a>) -> Result<Self> {
        let mut stream = pkg_length::parse_to_stream(stream)?;

        let name = NameString::parse(&mut stream)?;
        let id = next!(stream);
        let address =
            u32::from_le_bytes([next!(stream), next!(stream), next!(stream), next!(stream)]);
        let length = next!(stream);
        let term_list = TermList::parse(stream);

        Ok(Processor {
            name,
            id,
            address,
            length,
            term_list,
        })
    }

    pub(crate) fn name(&self) -> &NameString {
        &self.name
    }

    pub(crate) fn id(&self) -> u8 {
        self.id
    }

    pub(crate) fn address(&self) -> u32 {
        self.address
    }

    pub(crate) fn length(&self) -> u8 {
        self.length
    }

    pub(crate) fn term_list(&mut self) -> &mut TermList<'a> {
        &mut self.term_list
    }
}
