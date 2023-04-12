use crate::parser::{ast::Argument, Context, Result, Stream};
use alloc::boxed::Box;

pub(crate) struct LNot<'a> {
    operand: Box<Argument<'a>>,
}

impl<'a> LNot<'a> {
    pub(super) fn parse(stream: &mut Stream<'a>, context: &mut Context) -> Result<Self> {
        let operand = Box::new(Argument::parse(stream, context)?);

        Ok(LNot { operand })
    }
}

impl<'a> core::fmt::Display for LNot<'a> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "LNot ({})", self.operand)
    }
}
