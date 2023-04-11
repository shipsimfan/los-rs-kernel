use super::{size_of::SizeOf, LLess, ReferenceTypeOp, Store, Subtract, ToBuffer, ToHexString};
use crate::parser::{next, Context, Result, Stream};

pub(crate) enum Expression<'a> {
    LLess(LLess<'a>),
    ReferenceTypeOp(ReferenceTypeOp<'a>),
    SizeOf(SizeOf<'a>),
    Store(Store<'a>),
    Subtract(Subtract<'a>),
    ToBuffer(ToBuffer<'a>),
    ToHexString(ToHexString<'a>),
}

const STORE_OP: u8 = 0x70;
const SUBTRACT_OP: u8 = 0x74;
const SIZE_OF_OP: u8 = 0x87;
const LLESS_OP: u8 = 0x95;
const TO_BUFFER_OP: u8 = 0x96;
const TO_HEX_STRING_OP: u8 = 0x98;

impl<'a> Expression<'a> {
    pub(in crate::parser::ast) fn parse(
        stream: &mut Stream<'a>,
        context: &mut Context,
    ) -> Result<Option<Self>> {
        if let Some(reference_type_op) = ReferenceTypeOp::parse(stream, context)? {
            return Ok(Some(Expression::ReferenceTypeOp(reference_type_op)));
        }

        match next!(stream, "Expression") {
            LLESS_OP => LLess::parse(stream, context).map(|lless| Expression::LLess(lless)),
            STORE_OP => Store::parse(stream, context).map(|store| Expression::Store(store)),
            SIZE_OF_OP => SizeOf::parse(stream, context).map(|size_of| Expression::SizeOf(size_of)),
            SUBTRACT_OP => {
                Subtract::parse(stream, context).map(|subtract| Expression::Subtract(subtract))
            }
            TO_BUFFER_OP => {
                ToBuffer::parse(stream, context).map(|to_buffer| Expression::ToBuffer(to_buffer))
            }
            TO_HEX_STRING_OP => ToHexString::parse(stream, context)
                .map(|to_hex_string| Expression::ToHexString(to_hex_string)),
            _ => {
                stream.prev();
                return Ok(None);
            }
        }
        .map(|expression| Some(expression))
    }
}

impl<'a> core::fmt::Display for Expression<'a> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Expression::LLess(lless) => lless.fmt(f),
            Expression::ReferenceTypeOp(reference_type_op) => reference_type_op.fmt(f),
            Expression::SizeOf(size_of) => size_of.fmt(f),
            Expression::Store(store) => store.fmt(f),
            Expression::Subtract(subtract) => subtract.fmt(f),
            Expression::ToBuffer(to_buffer) => to_buffer.fmt(f),
            Expression::ToHexString(to_hex_string) => to_hex_string.fmt(f),
        }
    }
}
