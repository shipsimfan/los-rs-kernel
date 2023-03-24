use crate::aml::{impl_core_display, Display, NameString, Result, Stream};

pub(in crate::aml::term_objects) struct Event {
    name: NameString,
}

impl Event {
    pub(super) fn parse(stream: &mut Stream) -> Result<Self> {
        let name = NameString::parse(stream)?;

        Ok(Event { name })
    }
}

impl Display for Event {
    fn display(&self, f: &mut core::fmt::Formatter, depth: usize, _: bool) -> core::fmt::Result {
        self.display_prefix(f, depth)?;
        writeln!(f, "Event ({})", self.name)
    }
}

impl_core_display!(Event);
