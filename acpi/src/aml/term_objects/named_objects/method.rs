use crate::aml::{
    impl_core_display, next, pkg_length, term_objects::TermList, Display, NameString, Result,
    Stream,
};

pub(in crate::aml::term_objects) struct Method {
    name: NameString,
    method_flags: u8,
    term_list: TermList,
}

impl Method {
    pub(super) fn parse(stream: &mut Stream) -> Result<Self> {
        let mut stream = pkg_length::parse_to_stream(stream)?;

        let name = NameString::parse(&mut stream)?;
        let method_flags = next!(stream);
        let term_list = TermList::parse(&mut stream)?;

        Ok(Method {
            name,
            method_flags,
            term_list,
        })
    }
}

impl Display for Method {
    fn display(&self, f: &mut core::fmt::Formatter, depth: usize, last: bool) -> core::fmt::Result {
        self.display_prefix(f, depth)?;
        write!(f, "Method ({}, {}) ", self.name, self.method_flags)?;

        self.term_list.display(f, depth, last)
    }
}

impl_core_display!(Method);
