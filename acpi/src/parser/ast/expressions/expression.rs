use super::{
    acquire::Acquire, size_of::SizeOf, Increment, LLess, ReferenceTypeOp, ShiftLeft, Store,
    Subtract, ToBuffer, ToHexString,
};
use crate::parser::{next, Context, Result, Stream};

pub(crate) enum Expression<'a> {
    Acquire(Acquire<'a>),
    Increment(Increment<'a>),
    LLess(LLess<'a>),
    ReferenceTypeOp(ReferenceTypeOp<'a>),
    ShiftLeft(ShiftLeft<'a>),
    SizeOf(SizeOf<'a>),
    Store(Store<'a>),
    Subtract(Subtract<'a>),
    ToBuffer(ToBuffer<'a>),
    ToHexString(ToHexString<'a>),
}

const STORE_OP: u8 = 0x70;
const SUBTRACT_OP: u8 = 0x74;
const INCREMENT_OP: u8 = 0x75;
const SHIFT_LEFT_OP: u8 = 0x79;
const SIZE_OF_OP: u8 = 0x87;
const LLESS_OP: u8 = 0x95;
const TO_BUFFER_OP: u8 = 0x96;
const TO_HEX_STRING_OP: u8 = 0x98;

const EXT_OP_PREFIX: u8 = 0x5B;

const ACQUIRE_OP: u8 = 0x23;

impl<'a> Expression<'a> {
    pub(in crate::parser::ast) fn parse(
        stream: &mut Stream<'a>,
        context: &mut Context,
    ) -> Result<Option<Self>> {
        if let Some(reference_type_op) = ReferenceTypeOp::parse(stream, context)? {
            return Ok(Some(Expression::ReferenceTypeOp(reference_type_op)));
        }

        match next!(stream, "Expression") {
            INCREMENT_OP => {
                Increment::parse(stream, context).map(|increment| Expression::Increment(increment))
            }
            LLESS_OP => LLess::parse(stream, context).map(|lless| Expression::LLess(lless)),
            SHIFT_LEFT_OP => ShiftLeft::parse(stream, context)
                .map(|shift_left| Expression::ShiftLeft(shift_left)),
            SIZE_OF_OP => SizeOf::parse(stream, context).map(|size_of| Expression::SizeOf(size_of)),
            STORE_OP => Store::parse(stream, context).map(|store| Expression::Store(store)),
            SUBTRACT_OP => {
                Subtract::parse(stream, context).map(|subtract| Expression::Subtract(subtract))
            }
            TO_BUFFER_OP => {
                ToBuffer::parse(stream, context).map(|to_buffer| Expression::ToBuffer(to_buffer))
            }
            TO_HEX_STRING_OP => ToHexString::parse(stream, context)
                .map(|to_hex_string| Expression::ToHexString(to_hex_string)),
            EXT_OP_PREFIX => match next!(stream, "Extended Expression") {
                ACQUIRE_OP => {
                    Acquire::parse(stream, context).map(|acquire| Expression::Acquire(acquire))
                }
                _ => {
                    stream.prev();
                    stream.prev();
                    return Ok(None);
                }
            },
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
            Expression::Acquire(acquire) => acquire.fmt(f),
            Expression::Increment(increment) => increment.fmt(f),
            Expression::LLess(lless) => lless.fmt(f),
            Expression::ReferenceTypeOp(reference_type_op) => reference_type_op.fmt(f),
            Expression::ShiftLeft(shift_left) => shift_left.fmt(f),
            Expression::SizeOf(size_of) => size_of.fmt(f),
            Expression::Store(store) => store.fmt(f),
            Expression::Subtract(subtract) => subtract.fmt(f),
            Expression::ToBuffer(to_buffer) => to_buffer.fmt(f),
            Expression::ToHexString(to_hex_string) => to_hex_string.fmt(f),
        }
    }
}
