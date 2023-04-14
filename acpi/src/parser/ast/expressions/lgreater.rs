use crate::parser::{ast::Argument, Context, Result, Stream};
use alloc::boxed::Box;

pub(crate) struct LGreater<'a> {
    operand1: Box<Argument<'a>>,
    operand2: Box<Argument<'a>>,
}

impl<'a> LGreater<'a> {
    pub(super) fn parse(stream: &mut Stream<'a>, context: &mut Context) -> Result<Self> {
        let operand1 = Box::new(Argument::parse(stream, context)?);
        let operand2 = Box::new(Argument::parse(stream, context)?);

        Ok(LGreater { operand1, operand2 })
    }
}

impl<'a> core::fmt::Display for LGreater<'a> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "LGreater ({}, {})", self.operand1, self.operand2)
    }
}
