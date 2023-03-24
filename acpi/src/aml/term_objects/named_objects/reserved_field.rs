use crate::aml::{impl_core_display, pkg_length, Display, Result, Stream};

pub(super) struct ReservedField {
    offset: usize,
}

impl ReservedField {
    pub(super) fn parse(stream: &mut Stream) -> Result<Self> {
        let offset = stream.offset() - 1;

        pkg_length::parse_to_stream(stream)?;

        Ok(ReservedField { offset })
    }
}

impl Display for ReservedField {
    fn display(&self, f: &mut core::fmt::Formatter, depth: usize, _: bool) -> core::fmt::Result {
        self.display_prefix(f, depth)?;
        writeln!(f, "Reserved Field @ {}", self.offset)
    }
}

impl_core_display!(ReservedField);
