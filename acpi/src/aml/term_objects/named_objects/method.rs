use crate::aml::{
    impl_core_display, next, pkg_length, term_objects::TermList, Display, NameString, Result,
    Stream,
};

pub(in crate::aml::term_objects) struct Method {
    name: NameString,
    method_flags: u8,
}

impl Method {
    pub(super) fn parse(stream: &mut Stream) -> Result<Self> {
        let mut stream = pkg_length::parse_to_stream(stream)?;

        let name = NameString::parse(&mut stream)?;
        let method_flags = next!(stream);
        //let term_list = TermList::parse(&mut stream)?;

        Ok(Method { name, method_flags })
    }
}

impl Display for Method {
    fn display(&self, f: &mut core::fmt::Formatter, depth: usize, _: bool) -> core::fmt::Result {
        self.display_prefix(f, depth)?;
        writeln!(f, "Method ({}, {})", self.name, self.method_flags)
    }
}

impl_core_display!(Method);
