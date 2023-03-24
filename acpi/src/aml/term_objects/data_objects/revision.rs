use crate::aml::{impl_core_display, Display, Result, Stream};

pub(in crate::aml::term_objects) struct Revision {}

impl Revision {
    pub(super) fn parse(_stream: &mut Stream) -> Result<Self> {
        Ok(Revision {})
    }
}

impl Display for Revision {
    fn display(&self, f: &mut core::fmt::Formatter, depth: usize, _: bool) -> core::fmt::Result {
        self.display_prefix(f, depth)?;
        writeln!(f, "Revision")
    }
}

impl_core_display!(Revision);
