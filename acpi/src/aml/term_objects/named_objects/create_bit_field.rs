use crate::aml::{impl_core_display, term_objects::TermArg, Display, NameString, Result, Stream};

pub(in crate::aml::term_objects) struct CreateBitField {
    name: NameString,
    source_buff: TermArg,
    bit_index: TermArg,
}

impl CreateBitField {
    pub(super) fn parse(stream: &mut Stream) -> Result<Self> {
        let source_buff = TermArg::parse(stream)?;
        let bit_index = TermArg::parse(stream)?;
        let name = NameString::parse(stream)?;

        Ok(CreateBitField {
            name,
            source_buff,
            bit_index,
        })
    }
}

impl Display for CreateBitField {
    fn display(&self, f: &mut core::fmt::Formatter, depth: usize, _: bool) -> core::fmt::Result {
        self.display_prefix(f, depth)?;
        writeln!(
            f,
            "Create Bit Field ({}, {}, {})",
            self.source_buff, self.bit_index, self.name
        )
    }
}

impl_core_display!(CreateBitField);
