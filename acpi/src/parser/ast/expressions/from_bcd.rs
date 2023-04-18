use crate::parser::{ast::Argument, Context, Result, Stream, SuperName};
use alloc::boxed::Box;

pub(crate) struct FromBCD<'a> {
    operand: Box<Argument<'a>>,
    target: SuperName<'a>,
}

impl<'a> FromBCD<'a> {
    pub(super) fn parse(stream: &mut Stream<'a>, context: &mut Context) -> Result<Self> {
        let operand = Box::new(Argument::parse(stream, context)?);
        let target = SuperName::parse(stream, context)?;

        Ok(FromBCD { operand, target })
    }
}

impl<'a> core::fmt::Display for FromBCD<'a> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "FromBCD ({}, {})", self.operand, self.target)
    }
}
