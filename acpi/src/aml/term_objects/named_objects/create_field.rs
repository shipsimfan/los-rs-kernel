use crate::aml::{impl_core_display, term_objects::TermArg, Display, NameString, Result, Stream};

pub(in crate::aml::term_objects) struct CreateField {
    name: NameString,
    source_buff: TermArg,
    bit_index: TermArg,
    num_bits: TermArg,
}

impl CreateField {
    pub(super) fn parse(stream: &mut Stream) -> Result<Self> {
        let source_buff = TermArg::parse(stream)?;
        let bit_index = TermArg::parse(stream)?;
        let num_bits = TermArg::parse(stream)?;
        let name = NameString::parse(stream)?;

        Ok(CreateField {
            name,
            source_buff,
            bit_index,
            num_bits,
        })
    }
}

impl Display for CreateField {
    fn display(&self, f: &mut core::fmt::Formatter, depth: usize, _: bool) -> core::fmt::Result {
        self.display_prefix(f, depth)?;
        writeln!(
            f,
            "Create Field ({}, {}, {}, {})",
            self.source_buff, self.bit_index, self.num_bits, self.name
        )
    }
}

impl_core_display!(CreateField);
