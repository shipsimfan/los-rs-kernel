use crate::aml::{impl_core_display, pkg_length, Display, NameString, Result, Stream};

pub(in crate::aml::term_objects) struct Field {
    offset: usize,
    name: NameString,
}

impl Field {
    pub(super) fn parse(stream: &mut Stream) -> Result<Self> {
        let offset = stream.offset() - 2;

        let mut stream = pkg_length::parse_to_stream(stream)?;

        let name = NameString::parse(&mut stream)?;

        // TODO: Collect field units

        Ok(Field { offset, name })
    }
}

impl Display for Field {
    fn display(&self, f: &mut core::fmt::Formatter, depth: usize) -> core::fmt::Result {
        self.display_prefix(f, depth)?;
        writeln!(f, "Field {} @ {}", self.name, self.offset)
    }
}

impl_core_display!(Field);
