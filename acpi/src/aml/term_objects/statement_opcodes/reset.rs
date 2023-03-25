use crate::aml::{impl_core_display, super_name::SuperName, Display, Result, Stream};

pub(in crate::aml::term_objects) struct Reset {
    event: SuperName,
}

impl Reset {
    pub(super) fn parse(stream: &mut Stream) -> Result<Self> {
        let event = SuperName::parse(stream)?;

        Ok(Reset { event })
    }
}

impl Display for Reset {
    fn display(&self, f: &mut core::fmt::Formatter, depth: usize, _: bool) -> core::fmt::Result {
        self.display_prefix(f, depth)?;
        writeln!(f, "Reset ({})", self.event)
    }
}

impl_core_display!(Reset);
