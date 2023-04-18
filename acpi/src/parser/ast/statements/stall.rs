use crate::{
    display_prefix, impl_core_display_lifetime,
    parser::{ast::Argument, Context, Result, Stream},
    Display,
};

pub(crate) struct Stall<'a> {
    time: Argument<'a>,
}

impl<'a> Stall<'a> {
    pub(super) fn parse(stream: &mut Stream<'a>, context: &mut Context) -> Result<Self> {
        let time = Argument::parse(stream, context)?;

        Ok(Stall { time })
    }
}

impl<'a> Display for Stall<'a> {
    fn display(
        &self,
        f: &mut core::fmt::Formatter,
        depth: usize,
        _: bool,
        newline: bool,
    ) -> core::fmt::Result {
        display_prefix!(f, depth);
        write!(f, "Stall ({})", self.time)?;

        if newline {
            writeln!(f)
        } else {
            Ok(())
        }
    }
}

impl_core_display_lifetime!(Stall);
