use super::{Buffer, ByteConst, ConstObj, String, WordConst};
use crate::aml::{impl_core_display, peek, Display, Result, Stream};

pub(in crate::aml::term_objects) enum ComputationalData {
    Buffer(Buffer),
    ByteConst(ByteConst),
    ConstObj(ConstObj),
    String(String),
    WordConst(WordConst),
}

const BYTE_PREFIX: u8 = 0x0A;
const WORD_PREFIX: u8 = 0x0B;
const STRING_PREFIX: u8 = 0x0D;
const BUFFER_PREFIX: u8 = 0x11;

impl ComputationalData {
    pub(super) fn parse(stream: &mut Stream) -> Result<Self> {
        match peek!(stream) {
            BYTE_PREFIX => {
                stream.next();
                ByteConst::parse(stream).map(|byte_const| ComputationalData::ByteConst(byte_const))
            }
            WORD_PREFIX => {
                stream.next();
                WordConst::parse(stream).map(|word_const| ComputationalData::WordConst(word_const))
            }
            STRING_PREFIX => {
                stream.next();
                String::parse(stream).map(|string| ComputationalData::String(string))
            }
            BUFFER_PREFIX => {
                stream.next();
                Buffer::parse(stream).map(|buffer| ComputationalData::Buffer(buffer))
            }
            _ => ConstObj::parse(stream).map(|const_obj| ComputationalData::ConstObj(const_obj)),
        }
    }
}

impl Display for ComputationalData {
    fn display(&self, f: &mut core::fmt::Formatter, depth: usize) -> core::fmt::Result {
        match self {
            ComputationalData::Buffer(buffer) => buffer.display(f, depth),
            ComputationalData::ByteConst(byte_const) => byte_const.display(f, depth),
            ComputationalData::ConstObj(const_obj) => const_obj.display(f, depth),
            ComputationalData::String(string) => string.display(f, depth),
            ComputationalData::WordConst(word_const) => word_const.display(f, depth),
        }
    }
}

impl_core_display!(ComputationalData);
