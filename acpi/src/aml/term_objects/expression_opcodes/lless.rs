use crate::aml::{impl_core_display, term_objects::TermArg, Display, Result, Stream};
use alloc::boxed::Box;

pub(in crate::aml::term_objects) struct LLess {
    operand1: Box<TermArg>,
    operand2: Box<TermArg>,
}

impl LLess {
    pub(super) fn parse(stream: &mut Stream) -> Result<Self> {
        let operand1 = Box::new(TermArg::parse(stream)?);
        let operand2 = Box::new(TermArg::parse(stream)?);

        Ok(LLess { operand1, operand2 })
    }
}

impl Display for LLess {
    fn display(&self, f: &mut core::fmt::Formatter, depth: usize, _: bool) -> core::fmt::Result {
        self.display_prefix(f, depth)?;
        write!(f, "LLess ({}, {})", self.operand1, self.operand2)
    }
}

impl_core_display!(LLess);
