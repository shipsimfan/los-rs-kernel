use crate::aml::{impl_core_display, Display, Result, Stream};

pub(in crate::aml::term_objects) struct Break {}

impl Break {
    pub(super) fn parse(_stream: &mut Stream) -> Result<Self> {
        Ok(Break {})
    }
}

impl Display for Break {
    fn display(&self, f: &mut core::fmt::Formatter, depth: usize, _: bool) -> core::fmt::Result {
        self.display_prefix(f, depth)?;
        writeln!(f, "Break")
    }
}

impl_core_display!(Break);
