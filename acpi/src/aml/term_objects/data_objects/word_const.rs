use crate::aml::{impl_core_display, next, Display, Result, Stream};

pub(in crate::aml::term_objects) struct WordConst {
    offset: usize,

    word: u16,
}

impl WordConst {
    pub(super) fn parse(stream: &mut Stream) -> Result<Self> {
        let offset = stream.offset() - 1;

        let word = u16::from_le_bytes([next!(stream), next!(stream)]);

        Ok(WordConst { offset, word })
    }
}

impl Display for WordConst {
    fn display(&self, f: &mut core::fmt::Formatter, depth: usize) -> core::fmt::Result {
        self.display_prefix(f, depth)?;
        writeln!(f, "Word Const @ {}: {:#04X}", self.offset, self.word)
    }
}

impl_core_display!(WordConst);
