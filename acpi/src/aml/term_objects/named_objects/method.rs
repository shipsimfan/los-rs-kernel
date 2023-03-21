use crate::aml::{impl_core_display, pkg_length, Display, NameString, Result, Stream};

pub(in crate::aml::term_objects) struct Method {
    offset: usize,

    name: NameString,
}

impl Method {
    pub(super) fn parse(stream: &mut Stream) -> Result<Self> {
        let offset = stream.offset() - 1;

        let mut stream = pkg_length::parse_to_stream(stream)?;

        let name = NameString::parse(&mut stream)?;

        Ok(Method { offset, name })
    }
}

impl Display for Method {
    fn display(&self, f: &mut core::fmt::Formatter, depth: usize) -> core::fmt::Result {
        self.display_prefix(f, depth)?;
        writeln!(f, "Method {} @ {}", self.name, self.offset)
    }
}

impl_core_display!(Method);
