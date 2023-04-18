use crate::{
    display_prefix, impl_core_display_lifetime,
    namespace::objects::FieldFlags,
    parser::{ast::Argument, name_string, next, pkg_length, Context, Result, Stream},
    Display, Path,
};

pub struct BankField<'a> {
    op_region: Path,
    path: Path,
    value: Argument<'a>,
    flags: FieldFlags,
    // TODO: Add FieldList
}

impl<'a> BankField<'a> {
    pub(super) fn parse(stream: &mut Stream<'a>, context: &mut Context) -> Result<Self> {
        let mut stream = pkg_length::parse_to_stream(stream, "Bank Field")?;

        let op_region = name_string::parse(&mut stream, "Bank Field")?;
        let path = name_string::parse(&mut stream, "Bank Field")?;
        let value = Argument::parse(&mut stream, context)?;
        let flags = FieldFlags::parse(next!(stream, "Bank Field"));

        Ok(BankField {
            op_region,
            path,
            value,
            flags,
        })
    }
}

impl<'a> Display for BankField<'a> {
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
            "BankField ({}, {}, {}, {})",
            self.op_region, self.path, self.value, self.flags
        )?;

        if newline {
            writeln!(f)
        } else {
            Ok(())
        }
    }
}

impl_core_display_lifetime!(BankField);
