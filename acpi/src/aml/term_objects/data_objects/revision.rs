use crate::aml::{impl_core_display, Display, Result, Stream};

pub(in crate::aml::term_objects) struct Revision {
    offset: usize,
}

impl Revision {
    pub(super) fn parse(stream: &mut Stream) -> Result<Self> {
        let offset = stream.offset() - 2;
        Ok(Revision { offset })
    }
}

impl Display for Revision {
    fn display(&self, f: &mut core::fmt::Formatter, depth: usize) -> core::fmt::Result {
        self.display_prefix(f, depth)?;
        writeln!(f, "Revision @ {}", self.offset)
    }
}

impl_core_display!(Revision);
