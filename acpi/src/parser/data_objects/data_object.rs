use super::{Buffer, Package, String};
use crate::parser::{
    next, Error, Result, Stream, BUFFER_OP, BYTE_PREFIX, DWORD_PREFIX, ONES_OP, ONE_OP, PACKAGE_OP,
    QWORD_PREFIX, STRING_PREFIX, WORD_PREFIX, ZERO_OP,
};

pub(crate) enum DataObject<'a> {
    Zero,
    One,
    Ones,
    Byte(u8),
    Word(u16),
    DWord(u32),
    QWord(u64),

    Buffer(Buffer<'a>),
    String(String<'a>),

    Package(Package<'a>),
    // TODO: Implement VarPackage
}

impl<'a> DataObject<'a> {
    pub(in crate::parser) fn parse(stream: &mut Stream<'a>) -> Result<Self> {
        Ok(Self::parse_opt(stream)?
            .ok_or_else(|| Error::unexpected_byte(next!(stream), stream.offset() - 1))
            .unwrap())
    }

    pub(in crate::parser) fn parse_opt(stream: &mut Stream<'a>) -> Result<Option<Self>> {
        match next!(stream) {
            ZERO_OP => Ok(DataObject::Zero),
            ONE_OP => Ok(DataObject::One),
            ONES_OP => Ok(DataObject::Ones),
            BYTE_PREFIX => Ok(DataObject::Byte(next!(stream))),
            WORD_PREFIX => Ok(DataObject::Word(u16::from_le_bytes([
                next!(stream),
                next!(stream),
            ]))),
            DWORD_PREFIX => Ok(DataObject::DWord(u32::from_le_bytes([
                next!(stream),
                next!(stream),
                next!(stream),
                next!(stream),
            ]))),
            QWORD_PREFIX => Ok(DataObject::QWord(u64::from_le_bytes([
                next!(stream),
                next!(stream),
                next!(stream),
                next!(stream),
                next!(stream),
                next!(stream),
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
            DataObject::Zero => write!(f, "0"),
            DataObject::One => write!(f, "1"),
            DataObject::Ones => write!(f, "0xFF"),
            DataObject::Byte(byte) => write!(f, "{:#04X}", byte),
            DataObject::Word(word) => write!(f, "{:#06X}", word),
            DataObject::DWord(d_word) => write!(f, "{:#010X}", d_word),
            DataObject::QWord(q_word) => write!(f, "{:#018X}", q_word),

            DataObject::Buffer(buffer) => buffer.fmt(f),
            DataObject::String(string) => string.fmt(f),

            DataObject::Package(package) => package.fmt(f),
        }
    }
}
