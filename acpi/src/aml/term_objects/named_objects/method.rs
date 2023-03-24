use crate::aml::{
    impl_core_display, next, pkg_length, term_objects::TermList, Display, NameString, Result,
    Stream,
};

pub(in crate::aml::term_objects) struct Method {
    offset: usize,
    name: NameString,
    method_flags: u8,
}

impl Method {
    pub(super) fn parse(stream: &mut Stream) -> Result<Self> {
        let offset = stream.offset() - 1;

        let mut stream = pkg_length::parse_to_stream(stream)?;

        let name = NameString::parse(&mut stream)?;
        let method_flags = next!(stream);
        let term_list = TermList::parse(&mut stream)?;

        Ok(Method {
            offset,
            name,
            method_flags,
        })
    }
}

impl Display for Method {
    fn display(&self, f: &mut core::fmt::Formatter, depth: usize) -> core::fmt::Result {
        self.display_prefix(f, depth)?;
        writeln!(
            f,
            "Method {} ({:#02X}) @ {}",
            self.name, self.method_flags, self.offset
        )
    }
}

impl_core_display!(Method);
