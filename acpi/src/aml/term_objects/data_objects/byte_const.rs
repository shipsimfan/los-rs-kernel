use crate::aml::{impl_core_display, next, Display, Result, Stream};

pub(in crate::aml::term_objects) struct ByteConst {
    offset: usize,

    byte: u8,
}

impl ByteConst {
    pub(super) fn parse(stream: &mut Stream) -> Result<Self> {
        let offset = stream.offset() - 1;

        let byte = next!(stream);

        Ok(ByteConst { offset, byte })
    }
}

impl Display for ByteConst {
    fn display(&self, f: &mut core::fmt::Formatter, depth: usize) -> core::fmt::Result {
        self.display_prefix(f, depth)?;
        writeln!(f, "Byte Const @ {}: {:#02X}", self.offset, self.byte)
    }
}

impl_core_display!(ByteConst);
