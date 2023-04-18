use crate::parser::{ast::Argument, Context, Result, Stream, SuperName};
use alloc::boxed::Box;

pub(crate) struct Concat<'a> {
    data1: Box<Argument<'a>>,
    data2: Box<Argument<'a>>,
    target: SuperName<'a>,
}

impl<'a> Concat<'a> {
    pub(super) fn parse(stream: &mut Stream<'a>, context: &mut Context) -> Result<Self> {
        let data1 = Box::new(Argument::parse(stream, context)?);
        let data2 = Box::new(Argument::parse(stream, context)?);
        let target = SuperName::parse(stream, context)?;

        Ok(Concat {
            data1,
            data2,
            target,
        })
    }
}

impl<'a> core::fmt::Display for Concat<'a> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "Concat ({}, {}, {})",
            self.data1, self.data2, self.target
        )
    }
}
