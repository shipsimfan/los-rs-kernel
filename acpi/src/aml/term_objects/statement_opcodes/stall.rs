use crate::aml::{impl_core_display, term_objects::TermArg, Display, Result, Stream};

pub(in crate::aml::term_objects) struct Stall {
    time: TermArg,
}

impl Stall {
    pub(super) fn parse(stream: &mut Stream) -> Result<Self> {
        let time = TermArg::parse(stream)?;

        Ok(Stall { time })
    }
}

impl Display for Stall {
    fn display(&self, f: &mut core::fmt::Formatter, depth: usize, _: bool) -> core::fmt::Result {
        self.display_prefix(f, depth)?;
        writeln!(f, "Stall ({})", self.time)
    }
}

impl_core_display!(Stall);
