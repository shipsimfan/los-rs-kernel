use crate::aml::{impl_core_display, pkg_length, Display, NameString, Result, Stream};

pub(in crate::aml::term_objects) struct Scope {
    offset: usize,

    name: NameString,
}

impl Scope {
    pub(super) fn parse(stream: &mut Stream) -> Result<Self> {
        let offset = stream.offset() - 1;

        let mut stream = pkg_length::parse_to_stream(stream)?;

        let name = NameString::parse(&mut stream)?;

        Ok(Scope { offset, name })
    }
}

impl Display for Scope {
    fn display(&self, f: &mut core::fmt::Formatter, depth: usize) -> core::fmt::Result {
        self.display_prefix(f, depth)?;
        writeln!(f, "Scope {} @ {}", self.name, self.offset)
    }
}

impl_core_display!(Scope);
