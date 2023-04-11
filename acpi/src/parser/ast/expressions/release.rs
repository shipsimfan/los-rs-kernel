use crate::parser::{Context, Result, Stream, SuperName};

pub(crate) struct Release<'a> {
    mutex: SuperName<'a>,
}

impl<'a> Release<'a> {
    pub(super) fn parse(stream: &mut Stream<'a>, context: &mut Context) -> Result<Self> {
        let mutex = SuperName::parse(stream, context)?;

        Ok(Release { mutex })
    }
}

impl<'a> core::fmt::Display for Release<'a> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Release ({})", self.mutex)
    }
}
