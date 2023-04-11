use crate::parser::{Context, Result, Stream, SuperName};

pub(crate) struct Increment<'a> {
    target: SuperName<'a>,
}

impl<'a> Increment<'a> {
    pub(super) fn parse(stream: &mut Stream<'a>, context: &mut Context) -> Result<Self> {
        let target = SuperName::parse(stream, context)?;

        Ok(Increment { target })
    }
}

impl<'a> core::fmt::Display for Increment<'a> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Increment ({})", self.target)
    }
}
