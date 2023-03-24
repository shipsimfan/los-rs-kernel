use super::{Buffer, ByteConst, ConstObj, DWordConst, QWordConst, Revision, String, WordConst};
use crate::aml::{impl_core_display, match_next, peek, Display, Result, Stream};

pub(in crate::aml::term_objects) enum ComputationalData {
    Buffer(Buffer),
    ByteConst(ByteConst),
    ConstObj(ConstObj),
    DWordConst(DWordConst),
    QWordConst(QWordConst),
    Revision(Revision),
    String(String),
    WordConst(WordConst),
}

const BYTE_PREFIX: u8 = 0x0A;
const WORD_PREFIX: u8 = 0x0B;
const DWORD_PREFIX: u8 = 0x0C;
const STRING_PREFIX: u8 = 0x0D;
const QWORD_PREFIX: u8 = 0x0E;
const BUFFER_PREFIX: u8 = 0x11;

const EXT_OP_PREFIX: u8 = 0x5B;
const REVISION_OP: u8 = 0x30;

impl ComputationalData {
    pub(super) fn parse(stream: &mut Stream) -> Result<Option<Self>> {
        match peek!(stream) {
            BYTE_PREFIX => {
                stream.next();
                ByteConst::parse(stream)
                    .map(|byte_const| Some(ComputationalData::ByteConst(byte_const)))
            }
            WORD_PREFIX => {
                stream.next();
                WordConst::parse(stream)
                    .map(|word_const| Some(ComputationalData::WordConst(word_const)))
            }
            DWORD_PREFIX => {
                stream.next();
                DWordConst::parse(stream)
                    .map(|d_word_const| Some(ComputationalData::DWordConst(d_word_const)))
            }
            STRING_PREFIX => {
                stream.next();
                String::parse(stream).map(|string| Some(ComputationalData::String(string)))
            }
            QWORD_PREFIX => {
                stream.next();
                QWordConst::parse(stream)
                    .map(|q_word_const| Some(ComputationalData::QWordConst(q_word_const)))
            }
            BUFFER_PREFIX => {
                stream.next();
                Buffer::parse(stream).map(|buffer| Some(ComputationalData::Buffer(buffer)))
            }
            EXT_OP_PREFIX => {
                stream.next();
                match_next!(stream,
                    REVISION_OP => Revision::parse(stream).map(|revision| Some(ComputationalData::Revision(revision)))
                )
            }
            _ => ConstObj::parse(stream)
                .map(|const_obj| const_obj.map(|const_obj| ComputationalData::ConstObj(const_obj))),
        }
    }
}

impl Display for ComputationalData {
    fn display(&self, f: &mut core::fmt::Formatter, depth: usize, last: bool) -> core::fmt::Result {
        match self {
            ComputationalData::Buffer(buffer) => buffer.display(f, depth, last),
            ComputationalData::ByteConst(byte_const) => byte_const.display(f, depth, last),
            ComputationalData::ConstObj(const_obj) => const_obj.display(f, depth, last),
            ComputationalData::DWordConst(d_word_const) => d_word_const.display(f, depth, last),
            ComputationalData::QWordConst(q_word_const) => q_word_const.display(f, depth, last),
            ComputationalData::Revision(revision) => revision.display(f, depth, last),
            ComputationalData::String(string) => string.display(f, depth, last),
            ComputationalData::WordConst(word_const) => word_const.display(f, depth, last),
        }
    }
}

impl_core_display!(ComputationalData);
