use crate::parser::{ast::Argument, Context, Result, Stream, SuperName};
use alloc::boxed::Box;

pub(crate) struct Mod<'a> {
    dividend: Box<Argument<'a>>,
    divisor: Box<Argument<'a>>,
    target: SuperName<'a>,
}

impl<'a> Mod<'a> {
    pub(super) fn parse(stream: &mut Stream<'a>, context: &mut Context) -> Result<Self> {
        let dividend = Box::new(Argument::parse(stream, context)?);
        let divisor = Box::new(Argument::parse(stream, context)?);
        let target = SuperName::parse(stream, context)?;

        Ok(Mod {
            dividend,
            divisor,
            target,
        })
    }
}

impl<'a> core::fmt::Display for Mod<'a> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "Mod ({}, {}, {})",
            self.dividend, self.divisor, self.target
        )
    }
}
