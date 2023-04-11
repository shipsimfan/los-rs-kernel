use crate::{
    display_prefix, impl_core_display_lifetime,
    parser::{ast::Argument, Context, Result, Stream},
    Display,
};

pub(crate) struct Return<'a> {
    arg: Argument<'a>,
}

impl<'a> Return<'a> {
    pub(super) fn parse(stream: &mut Stream<'a>, context: &mut Context) -> Result<Self> {
        let arg = Argument::parse(stream, context)?;

        Ok(Return { arg })
    }
}

impl<'a> Display for Return<'a> {
    fn display(&self, f: &mut core::fmt::Formatter, depth: usize, _: bool) -> core::fmt::Result {
        display_prefix!(f, depth);
        writeln!(f, "Return ({})", self.arg)
    }
}

impl_core_display_lifetime!(Return);
