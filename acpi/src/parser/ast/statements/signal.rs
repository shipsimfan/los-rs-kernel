use crate::{
    display_prefix, impl_core_display_lifetime,
    parser::{Context, Result, Stream, SuperName},
    Display,
};

pub(crate) struct Signal<'a> {
    event: SuperName<'a>,
}

impl<'a> Signal<'a> {
    pub(super) fn parse(stream: &mut Stream<'a>, context: &mut Context) -> Result<Self> {
        let event = SuperName::parse(stream, context)?;

        Ok(Signal { event })
    }
}

impl<'a> Display for Signal<'a> {
    fn display(
        &self,
        f: &mut core::fmt::Formatter,
        depth: usize,
        _: bool,
        newline: bool,
    ) -> core::fmt::Result {
        display_prefix!(f, depth);
        write!(f, "Signal ({})", self.event)?;

        if newline {
            writeln!(f)
        } else {
            Ok(())
        }
    }
}

impl_core_display_lifetime!(Signal);
