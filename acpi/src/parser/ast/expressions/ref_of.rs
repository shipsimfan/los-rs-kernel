use crate::parser::{Context, Result, Stream, SuperName};
use alloc::boxed::Box;

pub(crate) struct RefOf<'a> {
    object: Box<SuperName<'a>>,
}

impl<'a> RefOf<'a> {
    pub(super) fn parse(stream: &mut Stream<'a>, context: &mut Context) -> Result<Self> {
        let object = Box::new(SuperName::parse(stream, context)?);

        Ok(RefOf { object })
    }
}

impl<'a> core::fmt::Display for RefOf<'a> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "RefOf ({})", self.object)
    }
}
