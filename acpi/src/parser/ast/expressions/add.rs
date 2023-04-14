use crate::parser::{ast::Argument, Context, Result, Stream, SuperName};
use alloc::boxed::Box;

pub(crate) struct Add<'a> {
    operand1: Box<Argument<'a>>,
    operand2: Box<Argument<'a>>,
    target: SuperName<'a>,
}

impl<'a> Add<'a> {
    pub(super) fn parse(stream: &mut Stream<'a>, context: &mut Context) -> Result<Self> {
        let operand1 = Box::new(Argument::parse(stream, context)?);
        let operand2 = Box::new(Argument::parse(stream, context)?);
        let target = SuperName::parse(stream, context)?;

        Ok(Add {
            operand1,
            operand2,
            target,
        })
    }
}

impl<'a> core::fmt::Display for Add<'a> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "Add ({}, {}, {})",
            self.operand1, self.operand2, self.target
        )
    }
}
