use crate::parser::{ast::Argument, Context, Result, Stream, SuperName};
use alloc::boxed::Box;

pub(crate) struct ShiftLeft<'a> {
    operand: Box<Argument<'a>>,
    shift_count: Box<Argument<'a>>,
    target: SuperName<'a>,
}

impl<'a> ShiftLeft<'a> {
    pub(super) fn parse(stream: &mut Stream<'a>, context: &mut Context) -> Result<Self> {
        let operand = Box::new(Argument::parse(stream, context)?);
        let shift_count = Box::new(Argument::parse(stream, context)?);
        let target = SuperName::parse(stream, context)?;

        Ok(ShiftLeft {
            operand,
            shift_count,
            target,
        })
    }
}

impl<'a> core::fmt::Display for ShiftLeft<'a> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "ShiftLeft ({}, {}, {})",
            self.operand, self.shift_count, self.target
        )
    }
}
