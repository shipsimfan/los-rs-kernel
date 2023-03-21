use super::{impl_core_display, term_objects::TermList, Display, Result, Stream};

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

impl Display for AML {
    fn display(&self, f: &mut core::fmt::Formatter, depth: usize) -> core::fmt::Result {
        self.display_prefix(f, depth)?;
        writeln!(f, "AML:")?;

        self.term_list.display(f, depth + 1)
    }
}

impl_core_display!(AML);
