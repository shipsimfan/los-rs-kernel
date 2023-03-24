use crate::aml::{impl_core_display, next, Display, Result, Stream};

pub(in crate::aml::term_objects) struct WordConst {
    word: u16,
}

impl WordConst {
    pub(super) fn parse(stream: &mut Stream) -> Result<Self> {
        let word = u16::from_le_bytes([next!(stream), next!(stream)]);

        Ok(WordConst { word })
    }
}

impl Display for WordConst {
    fn display(&self, f: &mut core::fmt::Formatter, depth: usize, _: bool) -> core::fmt::Result {
        self.display_prefix(f, depth)?;
        write!(f, "{:#04X}", self.word)
    }
}

impl_core_display!(WordConst);
