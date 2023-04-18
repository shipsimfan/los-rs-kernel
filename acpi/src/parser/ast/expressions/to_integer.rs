use crate::parser::{ast::Argument, Context, Result, Stream, SuperName};
use alloc::boxed::Box;

pub(crate) struct ToInteger<'a> {
    operand: Box<Argument<'a>>,
    target: SuperName<'a>,
}

impl<'a> ToInteger<'a> {
    pub(super) fn parse(stream: &mut Stream<'a>, context: &mut Context) -> Result<Self> {
        let operand = Box::new(Argument::parse(stream, context)?);
        let target = SuperName::parse(stream, context)?;

        Ok(ToInteger { operand, target })
    }
}

impl<'a> core::fmt::Display for ToInteger<'a> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "ToInteger ({}, {})", self.operand, self.target)
    }
}
