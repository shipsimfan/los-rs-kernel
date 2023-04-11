use crate::parser::{ast::Argument, Context, Result, Stream, SuperName};
use alloc::boxed::Box;

pub(crate) struct Store<'a> {
    operand: Box<Argument<'a>>,
    target: SuperName<'a>,
}

impl<'a> Store<'a> {
    pub(super) fn parse(stream: &mut Stream<'a>, context: &mut Context) -> Result<Self> {
        let operand = Box::new(Argument::parse(stream, context)?);
        let target = SuperName::parse(stream, context)?;

        Ok(Store { operand, target })
    }
}

impl<'a> core::fmt::Display for Store<'a> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Store ({}, {})", self.operand, self.target)
    }
}
