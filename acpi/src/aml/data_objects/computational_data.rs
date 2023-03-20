use crate::aml::{next, ASTNode, ByteStream, Result};

pub(in crate::aml) enum ComputationalData {
    One,
    Word(u16),
}

const ONE_OP: u8 = 0x01;

const WORD_PREFIX: u8 = 0x0B;

impl ComputationalData {
    pub(super) fn parse(stream: &mut ByteStream) -> Result<Self> {
        let c = next!(stream);
        match c {
            ONE_OP => Ok(ComputationalData::One),
            WORD_PREFIX => Ok(ComputationalData::Word(u16::from_le_bytes([
                next!(stream),
                next!(stream),
            ]))),
            _ => Err(crate::aml::Error::UnexpectedByte(c)),
        }
    }
}

impl ASTNode for ComputationalData {
    fn display(&self, f: &mut core::fmt::Formatter, depth: usize) -> core::fmt::Result {
        self.prefix_depth(f, depth)?;
        match self {
            ComputationalData::One => write!(f, "1"),
            ComputationalData::Word(word) => writeln!(f, "{}", word),
        }
    }
}
