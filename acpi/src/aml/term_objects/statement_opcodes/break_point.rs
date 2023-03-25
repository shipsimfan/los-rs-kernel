use crate::aml::{impl_core_display, Display, Result, Stream};

pub(in crate::aml::term_objects) struct BreakPoint {}

impl BreakPoint {
    pub(super) fn parse(_stream: &mut Stream) -> Result<Self> {
        Ok(BreakPoint {})
    }
}

impl Display for BreakPoint {
    fn display(&self, f: &mut core::fmt::Formatter, depth: usize, _: bool) -> core::fmt::Result {
        self.display_prefix(f, depth)?;
        writeln!(f, "Break Point")
    }
}

impl_core_display!(BreakPoint);
