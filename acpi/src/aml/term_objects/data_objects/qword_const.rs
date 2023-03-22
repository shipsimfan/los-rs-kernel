use crate::aml::{impl_core_display, next, Display, Result, Stream};

pub(in crate::aml::term_objects) struct QWordConst {
    offset: usize,

    q_word: u64,
}

impl QWordConst {
    pub(super) fn parse(stream: &mut Stream) -> Result<Self> {
        let offset = stream.offset() - 1;

        let q_word = u64::from_le_bytes([
            next!(stream),
            next!(stream),
            next!(stream),
            next!(stream),
            next!(stream),
            next!(stream),
            next!(stream),
            next!(stream),
        ]);

        Ok(QWordConst { offset, q_word })
    }
}

impl Display for QWordConst {
    fn display(&self, f: &mut core::fmt::Formatter, depth: usize) -> core::fmt::Result {
        self.display_prefix(f, depth)?;
        writeln!(f, "QWord Const @ {}: {:#016X}", self.offset, self.q_word)
    }
}

impl_core_display!(QWordConst);
