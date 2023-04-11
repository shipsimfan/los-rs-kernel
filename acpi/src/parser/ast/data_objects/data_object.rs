use super::{buffer::Buffer, byte, dword, qword, string, word};
use crate::{
    parser::{match_next, Context, Result, Stream},
    String,
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
    String(String),
}

const ZERO_OP: u8 = 0x00;
const ONE_OP: u8 = 0x01;
const BYTE_PREFIX: u8 = 0x0A;
const WORD_PREFIX: u8 = 0x0B;
const DWORD_PREFIX: u8 = 0x0C;
const STRING_PREFIX: u8 = 0x0D;
const QWORD_PREFIX: u8 = 0x0E;
const BUFFER_OP: u8 = 0x11;
const ONES_OP: u8 = 0xFF;

impl<'a> DataObject<'a> {
    pub(in crate::parser::ast) fn parse(
        stream: &mut Stream<'a>,
        context: &mut Context,
    ) -> Result<Self> {
        match_next!(stream, "Data Object",
            ZERO_OP => Ok(DataObject::Zero),
            ONE_OP => Ok(DataObject::One),
            ONES_OP => Ok(DataObject::Ones),

            BYTE_PREFIX => byte::parse(stream).map(|byte| DataObject::Byte(byte)),
            WORD_PREFIX => word::parse(stream).map(|word| DataObject::Word(word)),
            DWORD_PREFIX => dword::parse(stream).map(|dword| DataObject::DWord(dword)),
            QWORD_PREFIX => qword::parse(stream).map(|qword| DataObject::QWord(qword)),

            BUFFER_OP => Buffer::parse(stream, context).map(|buffer| DataObject::Buffer(buffer)),
            STRING_PREFIX => string::parse(stream).map(|string| DataObject::String(string)),
        )
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
            DataObject::DWord(dword) => write!(f, "{:#010X}", dword),
            DataObject::QWord(qword) => write!(f, "{:#018X}", qword),

            DataObject::Buffer(buffer) => buffer.fmt(f),
            DataObject::String(string) => string.fmt(f),
        }
    }
}
