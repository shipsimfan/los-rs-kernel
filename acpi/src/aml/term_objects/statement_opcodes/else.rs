use crate::aml::{impl_core_display, pkg_length, term_objects::TermList, Display, Result, Stream};

pub(in crate::aml::term_objects) struct Else {
    term_list: TermList,
}

impl Else {
    pub(super) fn parse(stream: &mut Stream) -> Result<Self> {
        let mut stream = pkg_length::parse_to_stream(stream)?;

        let term_list = TermList::parse(&mut stream)?;

        Ok(Else { term_list })
    }
}

impl Display for Else {
    fn display(&self, f: &mut core::fmt::Formatter, depth: usize, last: bool) -> core::fmt::Result {
        self.display_prefix(f, depth)?;
        write!(f, "Else ")?;

        self.term_list.display(f, depth, last)
    }
}

impl_core_display!(Else);
