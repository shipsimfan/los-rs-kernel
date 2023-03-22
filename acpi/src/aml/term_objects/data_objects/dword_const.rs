use crate::aml::{impl_core_display, next, Display, Result, Stream};

pub(in crate::aml::term_objects) struct DWordConst {
    offset: usize,

    d_word: u32,
}

impl DWordConst {
    pub(super) fn parse(stream: &mut Stream) -> Result<Self> {
        let offset = stream.offset() - 1;

        let d_word =
            u32::from_le_bytes([next!(stream), next!(stream), next!(stream), next!(stream)]);

        Ok(DWordConst { offset, d_word })
    }
}

impl Display for DWordConst {
    fn display(&self, f: &mut core::fmt::Formatter, depth: usize) -> core::fmt::Result {
        self.display_prefix(f, depth)?;
        writeln!(f, "DWord Const @ {}: {:#08X}", self.offset, self.d_word)
    }
}

impl_core_display!(DWordConst);
