use super::{term_objects::TermList, Display, Result, Stream};
use alloc::vec::Vec;

pub(crate) struct AML {
    term_list: TermList,
}

impl AML {
    pub(crate) fn parse(definition_block: &[u8]) -> Result<Self> {
        let mut stream = Stream::new(definition_block, 0);

        let term_list = TermList::parse(&mut stream)?;

        Ok(AML { term_list })
    }
}

impl core::fmt::Display for AML {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        writeln!(f, "AML:")?;
        self.term_list.display(f, 0)
    }
}
