use crate::parser::{ast::Argument, Context, Result, Stream, SuperName};
use alloc::boxed::Box;

pub(crate) struct Index<'a> {
    object: Box<Argument<'a>>,
    value: Box<Argument<'a>>,
    target: Box<SuperName<'a>>,
}

impl<'a> Index<'a> {
    pub(super) fn parse(stream: &mut Stream<'a>, context: &mut Context) -> Result<Self> {
        let object = Box::new(Argument::parse(stream, context)?);
        let value = Box::new(Argument::parse(stream, context)?);
        let target = Box::new(SuperName::parse(stream, context)?);

        Ok(Index {
            object,
            value,
            target,
        })
    }
}

impl<'a> core::fmt::Display for Index<'a> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "Index ({}, {}, {})",
            self.object, self.value, self.target
        )
    }
}
