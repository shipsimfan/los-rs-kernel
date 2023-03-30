use crate::parser::{match_next, next, Result, Stream, BYTE_PREFIX, ONE_OP, WORD_PREFIX};

pub(crate) enum DataObject {
    One,
    Byte(u8),
    Word(u16),
}

impl DataObject {
    pub(in crate::parser) fn parse(stream: &mut Stream) -> Result<Self> {
        match_next!(stream,
            ONE_OP => Ok(DataObject::One)
            BYTE_PREFIX => Ok(DataObject::Byte(next!(stream)))
            WORD_PREFIX => Ok(DataObject::Word(u16::from_le_bytes([next!(stream), next!(stream)])))
        )
    }
}

impl core::fmt::Display for DataObject {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            DataObject::One => write!(f, "1"),
            DataObject::Byte(byte) => write!(f, "{:#04X}", byte),
            DataObject::Word(word) => write!(f, "{:#06X}", word),
        }
    }
}
