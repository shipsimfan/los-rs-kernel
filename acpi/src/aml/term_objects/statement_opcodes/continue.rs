use crate::aml::{impl_core_display, Display, Result, Stream};

pub(in crate::aml::term_objects) struct Continue {}

impl Continue {
    pub(super) fn parse(_stream: &mut Stream) -> Result<Self> {
        Ok(Continue {})
    }
}

impl Display for Continue {
    fn display(&self, f: &mut core::fmt::Formatter, depth: usize, _: bool) -> core::fmt::Result {
        self.display_prefix(f, depth)?;
        writeln!(f, "Continue")
    }
}

impl_core_display!(Continue);
