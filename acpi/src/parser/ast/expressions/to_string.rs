use crate::parser::{ast::Argument, Context, Result, Stream, SuperName};
use alloc::boxed::Box;

pub(crate) struct ToString<'a> {
    data: Box<Argument<'a>>,
    length: Box<Argument<'a>>,
    target: SuperName<'a>,
}

impl<'a> ToString<'a> {
    pub(super) fn parse(stream: &mut Stream<'a>, context: &mut Context) -> Result<Self> {
        let data = Box::new(Argument::parse(stream, context)?);
        let length = Box::new(Argument::parse(stream, context)?);
        let target = SuperName::parse(stream, context)?;

        Ok(ToString {
            data,
            length,
            target,
        })
    }
}

impl<'a> core::fmt::Display for ToString<'a> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "ToString ({}, {}, {})",
            self.data, self.length, self.target
        )
    }
}
