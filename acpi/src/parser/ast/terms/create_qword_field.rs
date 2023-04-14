use crate::{
    display_prefix, impl_core_display_lifetime,
    parser::{ast::Argument, name_string, Context, Result, Stream},
    Display, Path,
};

pub(crate) struct CreateQWordField<'a> {
    source_buffer: Argument<'a>,
    byte_index: Argument<'a>,
    path: Path,
}

impl<'a> CreateQWordField<'a> {
    pub(super) fn parse(stream: &mut Stream<'a>, context: &mut Context) -> Result<Self> {
        let source_buffer = Argument::parse(stream, context)?;
        let byte_index = Argument::parse(stream, context)?;
        let path = name_string::parse(stream, "Create QWord Field")?;

        Ok(CreateQWordField {
            source_buffer,
            byte_index,
            path,
        })
    }
}

impl<'a> Display for CreateQWordField<'a> {
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
            "CreateQWordField ({}, {}, {})",
            self.source_buffer, self.byte_index, self.path
        )?;

        if newline {
            writeln!(f)
        } else {
            Ok(())
        }
    }
}

impl_core_display_lifetime!(CreateQWordField);
