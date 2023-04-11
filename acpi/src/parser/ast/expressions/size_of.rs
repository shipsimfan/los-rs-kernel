use crate::parser::{Context, Result, Stream, SuperName};

pub(crate) struct SizeOf<'a> {
    name: SuperName<'a>,
}

impl<'a> SizeOf<'a> {
    pub(super) fn parse(stream: &mut Stream<'a>, context: &mut Context) -> Result<Self> {
        let name = SuperName::parse(stream, context)?;

        Ok(SizeOf { name })
    }
}

impl<'a> core::fmt::Display for SizeOf<'a> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "SizeOf ({})", self.name)
    }
}
