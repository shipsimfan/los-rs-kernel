use super::TermList;
use crate::parser::{pkg_length, NameString, Result, Stream};

pub(crate) struct Scope<'a> {
    name: NameString,
    term_list: TermList<'a>,
}

impl<'a> Scope<'a> {
    pub(super) fn parse(stream: &mut Stream<'a>) -> Result<Self> {
        let mut stream = pkg_length::parse_to_stream(stream)?;

        let name = NameString::parse(&mut stream)?;
        let term_list = TermList::parse(stream);

        Ok(Scope { name, term_list })
    }

    pub(crate) fn name(&self) -> &NameString {
        &self.name
    }
}
