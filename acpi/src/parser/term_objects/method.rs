use super::TermList;
use crate::parser::{next, pkg_length, NameString, Result, Stream};

pub(crate) struct Method<'a> {
    name: NameString,
    arg_count: u8,
    serialized: bool,
    sync_level: u8,
    term_list: TermList<'a>,
}

fn parse_flags(flags: u8) -> (u8, bool, u8) {
    (
        flags & 7,
        flags.wrapping_shr(3) & 1 != 0,
        flags.wrapping_shr(4) & 0xF,
    )
}

impl<'a> Method<'a> {
    pub(super) fn parse(stream: &mut Stream<'a>) -> Result<Self> {
        let mut stream = pkg_length::parse_to_stream(stream)?;

        let name = NameString::parse(&mut stream)?;
        let (arg_count, serialized, sync_level) = parse_flags(next!(stream));
        let term_list = TermList::parse(stream);

        Ok(Method {
            name,
            arg_count,
            serialized,
            sync_level,
            term_list,
        })
    }

    pub(crate) fn name(&self) -> &NameString {
        &self.name
    }

    pub(crate) fn arg_count(&self) -> u8 {
        self.arg_count
    }

    pub(crate) fn serialized(&self) -> bool {
        self.serialized
    }

    pub(crate) fn sync_level(&self) -> u8 {
        self.sync_level
    }

    pub(crate) fn term_list(&self) -> &TermList<'a> {
        &self.term_list
    }
}
