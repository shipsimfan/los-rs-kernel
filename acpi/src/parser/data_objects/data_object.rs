use super::{Buffer, String};
use crate::parser::{
    match_next, next, Result, Stream, BUFFER_OP, BYTE_PREFIX, ONE_OP, STRING_PREFIX, WORD_PREFIX,
};

pub(crate) enum DataObject<'a> {
    One,
    Byte(u8),
    Word(u16),
    Buffer(Buffer<'a>),
    String(String<'a>),
}

impl<'a> DataObject<'a> {
    pub(in crate::parser) fn parse(stream: &mut Stream<'a>) -> Result<Self> {
        match_next!(stream,
            ONE_OP => Ok(DataObject::One)
            BYTE_PREFIX => Ok(DataObject::Byte(next!(stream)))
            WORD_PREFIX => Ok(DataObject::Word(u16::from_le_bytes([next!(stream), next!(stream)])))
            BUFFER_OP => Buffer::parse(stream).map(|buffer| DataObject::Buffer(buffer))
            STRING_PREFIX => String::parse(stream).map(|string| DataObject::String(string))
        )
    }
}

impl<'a> core::fmt::Display for DataObject<'a> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            DataObject::One => write!(f, "1"),
            DataObject::Byte(byte) => write!(f, "{:#04X}", byte),
            DataObject::Word(word) => write!(f, "{:#06X}", word),
            DataObject::Buffer(buffer) => buffer.fmt(f),
            DataObject::String(string) => string.fmt(f),
        }
    }
}
