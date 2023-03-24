use crate::aml::{impl_core_display, next, Display, Result, Stream};

pub(in crate::aml::term_objects) struct ByteConst {
    byte: u8,
}

impl ByteConst {
    pub(super) fn parse(stream: &mut Stream) -> Result<Self> {
        let byte = next!(stream);

        Ok(ByteConst { byte })
    }
}

impl Display for ByteConst {
    fn display(&self, f: &mut core::fmt::Formatter, depth: usize, _: bool) -> core::fmt::Result {
        self.display_prefix(f, depth)?;
        write!(f, "{:#02X}", self.byte)
    }
}

impl_core_display!(ByteConst);
