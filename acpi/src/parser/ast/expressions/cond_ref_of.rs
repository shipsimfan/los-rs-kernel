use crate::parser::{Context, Result, Stream, SuperName};

pub(crate) struct CondRefOf<'a> {
    name: SuperName<'a>,
    target: SuperName<'a>,
}

impl<'a> CondRefOf<'a> {
    pub(super) fn parse(stream: &mut Stream<'a>, context: &mut Context) -> Result<Self> {
        let name = SuperName::parse(stream, context)?;
        let target = SuperName::parse(stream, context)?;

        Ok(CondRefOf { name, target })
    }
}

impl<'a> core::fmt::Display for CondRefOf<'a> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "CondRefOf ({}, {})", self.name, self.target)
    }
}
