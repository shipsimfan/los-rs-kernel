use crate::aml::{impl_core_display, term_objects::TermArg, Display, Result, Stream};
use alloc::boxed::Box;

pub(in crate::aml) struct DerefOf {
    object: Box<TermArg>,
}

impl DerefOf {
    pub(super) fn parse(stream: &mut Stream) -> Result<Self> {
        let object = Box::new(TermArg::parse(stream)?);

        Ok(DerefOf { object })
    }
}

impl Display for DerefOf {
    fn display(&self, f: &mut core::fmt::Formatter, depth: usize, _: bool) -> core::fmt::Result {
        self.display_prefix(f, depth)?;
        write!(f, "Deref Of ({})", self.object)
    }
}

impl_core_display!(DerefOf);
