use crate::aml::{
    impl_core_display, pkg_length,
    term_objects::{TermArg, TermList},
    Display, Result, Stream,
};

pub(in crate::aml::term_objects) struct While {
    predicate: TermArg,
    term_list: TermList,
}

impl While {
    pub(super) fn parse(stream: &mut Stream) -> Result<Self> {
        let mut stream = pkg_length::parse_to_stream(stream)?;

        let predicate = TermArg::parse(&mut stream)?;
        let term_list = TermList::parse(&mut stream)?;

        Ok(While {
            predicate,
            term_list,
        })
    }
}

impl Display for While {
    fn display(&self, f: &mut core::fmt::Formatter, depth: usize, last: bool) -> core::fmt::Result {
        self.display_prefix(f, depth)?;
        write!(f, "While ({}) ", self.predicate)?;

        self.term_list.display(f, depth, last)
    }
}

impl_core_display!(While);
