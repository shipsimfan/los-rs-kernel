use crate::{
    display_prefix, impl_core_display_lifetime,
    parser::{ast::Argument, name_string, Context, Result, Stream},
    Display, Path,
};

pub(crate) struct DataRegion<'a> {
    region_name: Path,
    signature: Argument<'a>,
    oem_id: Argument<'a>,
    oem_table_id: Argument<'a>,
}

impl<'a> DataRegion<'a> {
    pub(super) fn parse(stream: &mut Stream<'a>, context: &mut Context) -> Result<Self> {
        let region_name = name_string::parse(stream, "Data Region")?;
        let signature = Argument::parse(stream, context)?;
        let oem_id = Argument::parse(stream, context)?;
        let oem_table_id = Argument::parse(stream, context)?;

        Ok(DataRegion {
            region_name,
            signature,
            oem_id,
            oem_table_id,
        })
    }
}

impl<'a> Display for DataRegion<'a> {
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
            "DataRegion ({}, {}, {}, {})",
            self.region_name, self.signature, self.oem_id, self.oem_table_id
        )?;

        if newline {
            writeln!(f)
        } else {
            Ok(())
        }
    }
}

impl_core_display_lifetime!(DataRegion);
