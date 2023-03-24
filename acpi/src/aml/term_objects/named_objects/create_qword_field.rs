use crate::aml::{impl_core_display, term_objects::TermArg, Display, NameString, Result, Stream};

pub(in crate::aml::term_objects) struct CreateQWordField {
    name: NameString,
    source_buff: TermArg,
    byte_index: TermArg,
}

impl CreateQWordField {
    pub(super) fn parse(stream: &mut Stream) -> Result<Self> {
        let source_buff = TermArg::parse(stream)?;
        let byte_index = TermArg::parse(stream)?;
        let name = NameString::parse(stream)?;

        Ok(CreateQWordField {
            name,
            source_buff,
            byte_index,
        })
    }
}

impl Display for CreateQWordField {
    fn display(&self, f: &mut core::fmt::Formatter, depth: usize, _: bool) -> core::fmt::Result {
        self.display_prefix(f, depth)?;
        writeln!(
            f,
            "Create QWord Field ({}, {}, {})",
            self.source_buff, self.byte_index, self.name
        )
    }
}

impl_core_display!(CreateQWordField);
