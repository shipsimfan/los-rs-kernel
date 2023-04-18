use crate::{
    display_prefix, impl_core_display_lifetime,
    parser::{ast::Argument, next, Context, Result, Stream},
    Display,
};

pub(crate) struct Fatal<'a> {
    r#type: u8,
    code: u32,
    argument: Argument<'a>,
}

impl<'a> Fatal<'a> {
    pub(super) fn parse(stream: &mut Stream<'a>, context: &mut Context) -> Result<Self> {
        let r#type = next!(stream, "Fatal");
        let code = u32::from_le_bytes([
            next!(stream, "Fatal"),
            next!(stream, "Fatal"),
            next!(stream, "Fatal"),
            next!(stream, "Fatal"),
        ]);
        let argument = Argument::parse(stream, context)?;

        Ok(Fatal {
            r#type,
            code,
            argument,
        })
    }
}

impl<'a> Display for Fatal<'a> {
    fn display(
        &self,
        f: &mut core::fmt::Formatter,
        depth: usize,
        _: bool,
        newline: bool,
    ) -> core::fmt::Result {
        display_prefix!(f, depth);
        write!(
            f,
            "Fatal ({}, {}, {})",
            self.r#type, self.code, self.argument
        )?;

        if newline {
            writeln!(f)
        } else {
            Ok(())
        }
    }
}

impl_core_display_lifetime!(Fatal);
