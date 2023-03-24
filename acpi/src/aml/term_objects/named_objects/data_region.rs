use crate::aml::{impl_core_display, term_objects::TermArg, Display, NameString, Result, Stream};

pub(in crate::aml::term_objects) struct DataRegion {
    name: NameString,
    signature_string: TermArg,
    oem_id_string: TermArg,
    oem_table_id_string: TermArg,
}

impl DataRegion {
    pub(super) fn parse(stream: &mut Stream) -> Result<Self> {
        let name = NameString::parse(stream)?;
        let term1 = TermArg::parse(stream)?;
        let term2 = TermArg::parse(stream)?;
        let term3 = TermArg::parse(stream)?;

        Ok(DataRegion {
            name,
            signature_string: term1,
            oem_id_string: term2,
            oem_table_id_string: term3,
        })
    }
}

impl Display for DataRegion {
    fn display(&self, f: &mut core::fmt::Formatter, depth: usize, _: bool) -> core::fmt::Result {
        self.display_prefix(f, depth)?;
        writeln!(
            f,
            "Data Table Region ({}, {}, {}, {})",
            self.name, self.signature_string, self.oem_id_string, self.oem_table_id_string
        )
    }
}

impl_core_display!(DataRegion);
