use crate::{
    display_prefix, impl_core_display_lifetime,
    parser::{ast::Argument, Context, Result, Stream},
    Display,
};

pub(crate) struct Sleep<'a> {
    time: Argument<'a>,
}

impl<'a> Sleep<'a> {
    pub(super) fn parse(stream: &mut Stream<'a>, context: &mut Context) -> Result<Self> {
        let time = Argument::parse(stream, context)?;

        Ok(Sleep { time })
    }
}

impl<'a> Display for Sleep<'a> {
    fn display(
        &self,
        f: &mut core::fmt::Formatter,
        depth: usize,
        _: bool,
        newline: bool,
    ) -> core::fmt::Result {
        display_prefix!(f, depth);
        write!(f, "Sleep ({})", self.time)?;

        if newline {
            writeln!(f)
        } else {
            Ok(())
        }
    }
}

impl_core_display_lifetime!(Sleep);
