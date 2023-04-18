use crate::parser::{Context, Result, Stream, SuperName};

pub(crate) struct Decrement<'a> {
    target: SuperName<'a>,
}

impl<'a> Decrement<'a> {
    pub(super) fn parse(stream: &mut Stream<'a>, context: &mut Context) -> Result<Self> {
        let target = SuperName::parse(stream, context)?;

        Ok(Decrement { target })
    }
}

impl<'a> core::fmt::Display for Decrement<'a> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Decrement ({})", self.target)
    }
}
