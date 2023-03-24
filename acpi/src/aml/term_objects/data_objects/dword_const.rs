use crate::aml::{impl_core_display, next, Display, Result, Stream};

pub(in crate::aml::term_objects) struct DWordConst {
    d_word: u32,
}

impl DWordConst {
    pub(super) fn parse(stream: &mut Stream) -> Result<Self> {
        let d_word =
            u32::from_le_bytes([next!(stream), next!(stream), next!(stream), next!(stream)]);

        Ok(DWordConst { d_word })
    }
}

impl Display for DWordConst {
    fn display(&self, f: &mut core::fmt::Formatter, depth: usize, _: bool) -> core::fmt::Result {
        self.display_prefix(f, depth)?;
        write!(f, "{:#08X}", self.d_word)
    }
}

impl_core_display!(DWordConst);
