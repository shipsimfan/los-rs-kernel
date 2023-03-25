use crate::aml::{impl_core_display, Display, Result, Stream};

pub(in crate::aml) struct Debug {}

impl Debug {
    pub(in crate::aml) fn parse(_: &mut Stream) -> Result<Self> {
        Ok(Debug {})
    }
}

impl Display for Debug {
    fn display(&self, f: &mut core::fmt::Formatter, depth: usize, _: bool) -> core::fmt::Result {
        self.display_prefix(f, depth)?;
        write!(f, "Debug")
    }
}

impl_core_display!(Debug);
