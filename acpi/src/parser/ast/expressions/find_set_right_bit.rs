use crate::parser::{ast::Argument, Context, Result, Stream, SuperName};
use alloc::boxed::Box;

pub(crate) struct FindSetRightBit<'a> {
    operand: Box<Argument<'a>>,
    target: SuperName<'a>,
}

impl<'a> FindSetRightBit<'a> {
    pub(super) fn parse(stream: &mut Stream<'a>, context: &mut Context) -> Result<Self> {
        let operand = Box::new(Argument::parse(stream, context)?);
        let target = SuperName::parse(stream, context)?;

        Ok(FindSetRightBit { operand, target })
    }
}

impl<'a> core::fmt::Display for FindSetRightBit<'a> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "FindSetRightBit ({}, {})", self.operand, self.target)
    }
}
