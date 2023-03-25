use crate::aml::{impl_core_display, term_objects::TermArg, Display, Result, Stream};

pub(in crate::aml::term_objects) struct Return {
    arg: TermArg,
}

impl Return {
    pub(super) fn parse(stream: &mut Stream) -> Result<Self> {
        let arg = TermArg::parse(stream)?;

        Ok(Return { arg })
    }
}

impl Display for Return {
    fn display(&self, f: &mut core::fmt::Formatter, depth: usize, _: bool) -> core::fmt::Result {
        self.display_prefix(f, depth)?;
        writeln!(f, "Return ({})", self.arg)
    }
}

impl_core_display!(Return);
