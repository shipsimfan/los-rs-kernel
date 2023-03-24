use crate::aml::{impl_core_display, term_objects::TermArg, Display, NameString, Result, Stream};

pub(in crate::aml::term_objects) struct CreateBitField {
    offset: usize,
    name: NameString,
    source_buff: TermArg,
    bit_index: TermArg,
}

impl CreateBitField {
    pub(super) fn parse(stream: &mut Stream) -> Result<Self> {
        let offset = stream.offset() - 1;

        let source_buff = TermArg::parse(stream)?;
        let bit_index = TermArg::parse(stream)?;
        let name = NameString::parse(stream)?;

        Ok(CreateBitField {
            offset,
            name,
            source_buff,
            bit_index,
        })
    }
}

impl Display for CreateBitField {
    fn display(&self, f: &mut core::fmt::Formatter, depth: usize) -> core::fmt::Result {
        self.display_prefix(f, depth)?;
        writeln!(f, "Create Bit Field {} @ {}", self.name, self.offset)?;

        self.display_prefix(f, depth + 1)?;
        writeln!(f, "Source Buffer:")?;
        self.source_buff.display(f, depth + 2)?;

        self.display_prefix(f, depth + 1)?;
        writeln!(f, "Bit Index:")?;
        self.bit_index.display(f, depth + 2)
    }
}

impl_core_display!(CreateBitField);
