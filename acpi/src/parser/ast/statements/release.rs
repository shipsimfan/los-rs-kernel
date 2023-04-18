use crate::{
    display_prefix, impl_core_display_lifetime,
    parser::{Context, Result, Stream, SuperName},
    Display,
};

pub(crate) struct Release<'a> {
    mutex: SuperName<'a>,
}

impl<'a> Release<'a> {
    pub(super) fn parse(stream: &mut Stream<'a>, context: &mut Context) -> Result<Self> {
        let mutex = SuperName::parse(stream, context)?;

        Ok(Release { mutex })
    }
}

impl<'a> Display for Release<'a> {
    fn display(
        &self,
        f: &mut core::fmt::Formatter,
        depth: usize,
        _: bool,
        newline: bool,
    ) -> core::fmt::Result {
        display_prefix!(f, depth);
        write!(f, "Release ({})", self.mutex)?;

        if newline {
            writeln!(f)
        } else {
            Ok(())
        }
    }
}

impl_core_display_lifetime!(Release);
