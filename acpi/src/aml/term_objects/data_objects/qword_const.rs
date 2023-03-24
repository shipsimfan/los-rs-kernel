use crate::aml::{impl_core_display, next, Display, Result, Stream};

pub(in crate::aml::term_objects) struct QWordConst {
    q_word: u64,
}

impl QWordConst {
    pub(super) fn parse(stream: &mut Stream) -> Result<Self> {
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

        Ok(QWordConst { q_word })
    }
}

impl Display for QWordConst {
    fn display(&self, f: &mut core::fmt::Formatter, depth: usize, _: bool) -> core::fmt::Result {
        self.display_prefix(f, depth)?;
        write!(f, "{:#016X}", self.q_word)
    }
}

impl_core_display!(QWordConst);
