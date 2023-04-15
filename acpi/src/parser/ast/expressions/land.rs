use crate::parser::{ast::Argument, Context, Result, Stream};
use alloc::boxed::Box;

pub(crate) struct LAnd<'a> {
    operand1: Box<Argument<'a>>,
    operand2: Box<Argument<'a>>,
}

impl<'a> LAnd<'a> {
    pub(super) fn parse(stream: &mut Stream<'a>, context: &mut Context) -> Result<Self> {
        let operand1 = Box::new(Argument::parse(stream, context)?);
        let operand2 = Box::new(Argument::parse(stream, context)?);

        Ok(LAnd { operand1, operand2 })
    }
}

impl<'a> core::fmt::Display for LAnd<'a> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Lor ({}, {})", self.operand1, self.operand2)
    }
}
