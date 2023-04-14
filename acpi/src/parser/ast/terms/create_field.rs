use crate::{
    display_prefix, impl_core_display_lifetime,
    parser::{ast::Argument, name_string, Context, Result, Stream},
    Display, Path,
};

pub(crate) struct CreateField<'a> {
    source_buffer: Argument<'a>,
    bit_index: Argument<'a>,
    num_bits: Argument<'a>,
    path: Path,
}

impl<'a> CreateField<'a> {
    pub(super) fn parse(stream: &mut Stream<'a>, context: &mut Context) -> Result<Self> {
        let source_buffer = Argument::parse(stream, context)?;
        let bit_index = Argument::parse(stream, context)?;
        let num_bits = Argument::parse(stream, context)?;
        let path = name_string::parse(stream, "Create Field")?;

        Ok(CreateField {
            source_buffer,
            bit_index,
            num_bits,
            path,
        })
    }
}

impl<'a> Display for CreateField<'a> {
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
            "CreateField ({}, {}, {}, {})",
            self.source_buffer, self.bit_index, self.num_bits, self.path
        )?;

        if newline {
            writeln!(f)
        } else {
            Ok(())
        }
    }
}

impl_core_display_lifetime!(CreateField);
