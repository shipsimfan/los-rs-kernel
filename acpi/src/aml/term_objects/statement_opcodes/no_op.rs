use crate::aml::{impl_core_display, Display, Result, Stream};

pub(in crate::aml::term_objects) struct NoOp {}

impl NoOp {
    pub(super) fn parse(_: &mut Stream) -> Result<Self> {
        Ok(NoOp {})
    }
}

impl Display for NoOp {
    fn display(&self, f: &mut core::fmt::Formatter, depth: usize, _: bool) -> core::fmt::Result {
        self.display_prefix(f, depth)?;
        writeln!(f, "No Op")
    }
}

impl_core_display!(NoOp);
