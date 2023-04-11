use crate::parser::{ast::Argument, Context, Result, Stream};
use alloc::boxed::Box;

pub(crate) struct DerefOf<'a> {
    object: Box<Argument<'a>>,
}

impl<'a> DerefOf<'a> {
    pub(super) fn parse(stream: &mut Stream<'a>, context: &mut Context) -> Result<Self> {
        let object = Box::new(Argument::parse(stream, context)?);

        Ok(DerefOf { object })
    }
}

impl<'a> core::fmt::Display for DerefOf<'a> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "DerefOf ({})", self.object)
    }
}
