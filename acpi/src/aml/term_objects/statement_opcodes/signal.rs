use crate::aml::{impl_core_display, super_name::SuperName, Display, Result, Stream};

pub(in crate::aml::term_objects) struct Signal {
    event: SuperName,
}

impl Signal {
    pub(super) fn parse(stream: &mut Stream) -> Result<Self> {
        let event = SuperName::parse(stream)?;

        Ok(Signal { event })
    }
}

impl Display for Signal {
    fn display(&self, f: &mut core::fmt::Formatter, depth: usize, _: bool) -> core::fmt::Result {
        self.display_prefix(f, depth)?;
        writeln!(f, "Signal ({})", self.event)
    }
}

impl_core_display!(Signal);
