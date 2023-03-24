use crate::aml::{impl_core_display, Display, NameString, Result, Stream};

pub(in crate::aml::term_objects) struct Event {
    offset: usize,
    name: NameString,
}

impl Event {
    pub(super) fn parse(stream: &mut Stream) -> Result<Self> {
        let offset = stream.offset() - 2;

        let name = NameString::parse(stream)?;

        Ok(Event { offset, name })
    }
}

impl Display for Event {
    fn display(&self, f: &mut core::fmt::Formatter, depth: usize) -> core::fmt::Result {
        self.display_prefix(f, depth)?;
        writeln!(f, "Event {} @ {}", self.name, self.offset)
    }
}

impl_core_display!(Event);
