use super::{Buffer, Package, String};
use crate::parser::{
    next, Error, Result, Stream, BUFFER_OP, BYTE_PREFIX, ONE_OP, PACKAGE_OP, STRING_PREFIX,
    WORD_PREFIX,
};

pub(crate) enum DataObject<'a> {
    One,
    Byte(u8),
    Word(u16),

    Buffer(Buffer<'a>),
    String(String<'a>),

    Package(Package<'a>),
}

impl<'a> DataObject<'a> {
    pub(in crate::parser) fn parse(stream: &mut Stream<'a>) -> Result<Self> {
        Self::parse_opt(stream)?
            .ok_or_else(|| Error::unexpected_byte(next!(stream), stream.offset() - 1))
    }

    pub(in crate::parser) fn parse_opt(stream: &mut Stream<'a>) -> Result<Option<Self>> {
        match next!(stream) {
            ONE_OP => Ok(DataObject::One),
            BYTE_PREFIX => Ok(DataObject::Byte(next!(stream))),
            WORD_PREFIX => Ok(DataObject::Word(u16::from_le_bytes([
                next!(stream),
                next!(stream),
            ]))),

            BUFFER_OP => Buffer::parse(stream).map(|buffer| DataObject::Buffer(buffer)),
            STRING_PREFIX => String::parse(stream).map(|string| DataObject::String(string)),

            PACKAGE_OP => Package::parse(stream).map(|package| DataObject::Package(package)),

            _ => {
                stream.prev();
                return Ok(None);
            }
        }
        .map(|result| Some(result))
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

            DataObject::Package(package) => package.fmt(f),
        }
    }
}
