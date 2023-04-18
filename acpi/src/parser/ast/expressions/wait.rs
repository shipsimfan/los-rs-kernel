use crate::parser::{ast::Argument, Context, Result, Stream, SuperName};
use alloc::boxed::Box;

pub(crate) struct Wait<'a> {
    event: SuperName<'a>,
    operand: Box<Argument<'a>>,
}

impl<'a> Wait<'a> {
    pub(super) fn parse(stream: &mut Stream<'a>, context: &mut Context) -> Result<Self> {
        let event = SuperName::parse(stream, context)?;
        let operand = Box::new(Argument::parse(stream, context)?);

        Ok(Wait { event, operand })
    }
}

impl<'a> core::fmt::Display for Wait<'a> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Wait ({}, {})", self.event, self.operand)
    }
}
