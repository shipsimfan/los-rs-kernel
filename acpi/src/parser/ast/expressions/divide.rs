use crate::parser::{ast::Argument, Context, Result, Stream, SuperName};
use alloc::boxed::Box;

pub(crate) struct Divide<'a> {
    dividend: Box<Argument<'a>>,
    divisor: Box<Argument<'a>>,
    remainder: SuperName<'a>,
    quotient: SuperName<'a>,
}

impl<'a> Divide<'a> {
    pub(super) fn parse(stream: &mut Stream<'a>, context: &mut Context) -> Result<Self> {
        let dividend = Box::new(Argument::parse(stream, context)?);
        let divisor = Box::new(Argument::parse(stream, context)?);
        let remainder = SuperName::parse(stream, context)?;
        let quotient = SuperName::parse(stream, context)?;

        Ok(Divide {
            dividend,
            divisor,
            remainder,
            quotient,
        })
    }
}

impl<'a> core::fmt::Display for Divide<'a> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "Divide ({}, {}, {}, {})",
            self.dividend, self.divisor, self.remainder, self.quotient
        )
    }
}
