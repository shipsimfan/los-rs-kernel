use crate::{
    display_prefix, impl_core_display_lifetime,
    parser::{ast::Argument, name_string, Context, Result, Stream},
    Display, Path,
};

pub(crate) struct CreateBitField<'a> {
    source_buffer: Argument<'a>,
    bit_index: Argument<'a>,
    path: Path,
}

impl<'a> CreateBitField<'a> {
    pub(super) fn parse(stream: &mut Stream<'a>, context: &mut Context) -> Result<Self> {
        let source_buffer = Argument::parse(stream, context)?;
        let bit_index = Argument::parse(stream, context)?;
        let path = name_string::parse(stream, "Create Bit Field")?;

        Ok(CreateBitField {
            source_buffer,
            bit_index,
            path,
        })
    }
}

impl<'a> Display for CreateBitField<'a> {
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
            "CreateBitField ({}, {}, {})",
            self.source_buffer, self.bit_index, self.path
        )?;

        if newline {
            writeln!(f)
        } else {
            Ok(())
        }
    }
}

impl_core_display_lifetime!(CreateBitField);
