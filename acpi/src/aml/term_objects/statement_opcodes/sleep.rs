use crate::aml::{impl_core_display, term_objects::TermArg, Display, Result, Stream};

pub(in crate::aml::term_objects) struct Sleep {
    time: TermArg,
}

impl Sleep {
    pub(super) fn parse(stream: &mut Stream) -> Result<Self> {
        let time = TermArg::parse(stream)?;

        Ok(Sleep { time })
    }
}

impl Display for Sleep {
    fn display(&self, f: &mut core::fmt::Formatter, depth: usize, _: bool) -> core::fmt::Result {
        self.display_prefix(f, depth)?;
        writeln!(f, "Sleep ({})", self.time)
    }
}

impl_core_display!(Sleep);
