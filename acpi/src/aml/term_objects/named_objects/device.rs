use crate::aml::{impl_core_display, pkg_length, Display, NameString, Result, Stream};

pub(in crate::aml::term_objects) struct Device {
    offset: usize,
    name: NameString,
}

impl Device {
    pub(super) fn parse(stream: &mut Stream) -> Result<Self> {
        let offset = stream.offset() - 2;

        let mut stream = pkg_length::parse_to_stream(stream)?;

        let name = NameString::parse(&mut stream)?;

        // TODO: Collect term list

        Ok(Device { offset, name })
    }
}

impl Display for Device {
    fn display(&self, f: &mut core::fmt::Formatter, depth: usize) -> core::fmt::Result {
        self.display_prefix(f, depth)?;
        writeln!(f, "Device {} @ {}", self.name, self.offset)
    }
}

impl_core_display!(Device);
