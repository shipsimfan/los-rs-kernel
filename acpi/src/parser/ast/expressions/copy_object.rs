use crate::parser::{Context, Result, SimpleName, Stream, SuperName};

pub(crate) struct CopyObject<'a> {
    name: SuperName<'a>,
    target: SimpleName,
}

impl<'a> CopyObject<'a> {
    pub(super) fn parse(stream: &mut Stream<'a>, context: &mut Context) -> Result<Self> {
        let name = SuperName::parse(stream, context)?;
        let target = SimpleName::parse(stream)?;

        Ok(CopyObject { name, target })
    }
}

impl<'a> core::fmt::Display for CopyObject<'a> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "CopyObject ({}, {})", self.name, self.target)
    }
}
