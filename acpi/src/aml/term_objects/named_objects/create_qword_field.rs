use crate::aml::{impl_core_display, term_objects::TermArg, Display, NameString, Result, Stream};

pub(in crate::aml::term_objects) struct CreateQWordField {
    offset: usize,
    name: NameString,
    source_buff: TermArg,
    byte_index: TermArg,
}

impl CreateQWordField {
    pub(super) fn parse(stream: &mut Stream) -> Result<Self> {
        let offset = stream.offset() - 1;

        let source_buff = TermArg::parse(stream)?;
        let byte_index = TermArg::parse(stream)?;
        let name = NameString::parse(stream)?;

        Ok(CreateQWordField {
            offset,
            name,
            source_buff,
            byte_index,
        })
    }
}

impl Display for CreateQWordField {
    fn display(&self, f: &mut core::fmt::Formatter, depth: usize) -> core::fmt::Result {
        self.display_prefix(f, depth)?;
        writeln!(f, "Create QWord Field {} @ {}", self.name, self.offset)?;

        self.display_prefix(f, depth + 1)?;
        writeln!(f, "Source Buffer:")?;
        self.source_buff.display(f, depth + 2)?;

        self.display_prefix(f, depth + 1)?;
        writeln!(f, "Byte Index:")?;
        self.byte_index.display(f, depth + 2)
    }
}

impl_core_display!(CreateQWordField);
