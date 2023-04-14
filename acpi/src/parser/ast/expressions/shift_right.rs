use crate::parser::{ast::Argument, Context, Result, Stream, SuperName};
use alloc::boxed::Box;

pub(crate) struct ShiftRight<'a> {
    operand: Box<Argument<'a>>,
    shift_count: Box<Argument<'a>>,
    target: SuperName<'a>,
}

impl<'a> ShiftRight<'a> {
    pub(super) fn parse(stream: &mut Stream<'a>, context: &mut Context) -> Result<Self> {
        let operand = Box::new(Argument::parse(stream, context)?);
        let shift_count = Box::new(Argument::parse(stream, context)?);
        let target = SuperName::parse(stream, context)?;

        Ok(ShiftRight {
            operand,
            shift_count,
            target,
        })
    }
}

impl<'a> core::fmt::Display for ShiftRight<'a> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "ShiftRight ({}, {}, {})",
            self.operand, self.shift_count, self.target
        )
    }
}
