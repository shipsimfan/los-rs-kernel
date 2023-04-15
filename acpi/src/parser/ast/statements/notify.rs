use crate::{
    display_prefix, impl_core_display_lifetime,
    parser::{ast::Argument, Context, Result, Stream, SuperName},
    Display,
};

pub(crate) struct Notify<'a> {
    object: SuperName<'a>,
    value: Argument<'a>,
}

impl<'a> Notify<'a> {
    pub(super) fn parse(stream: &mut Stream<'a>, context: &mut Context) -> Result<Self> {
        let object = SuperName::parse(stream, context)?;
        let value = Argument::parse(stream, context)?;

        Ok(Notify { object, value })
    }
}

impl<'a> Display for Notify<'a> {
    fn display(
        &self,
        f: &mut core::fmt::Formatter,
        depth: usize,
        _: bool,
        newline: bool,
    ) -> core::fmt::Result {
        display_prefix!(f, depth);
        write!(f, "Notify ({}, {})", self.object, self.value)?;

        if newline {
            writeln!(f)
        } else {
            Ok(())
        }
    }
}

impl_core_display_lifetime!(Notify);
