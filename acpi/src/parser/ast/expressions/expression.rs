use super::{
    acquire::Acquire, size_of::SizeOf, Increment, LEqual, LLess, LNot, MethodInvocation,
    ReferenceTypeOp, Release, ShiftLeft, Store, Subtract, ToBuffer, ToHexString,
};
use crate::parser::{match_next, next, Context, Error, Result, Stream};

pub(crate) enum Expression<'a> {
    Acquire(Acquire<'a>),
    Increment(Increment<'a>),
    LEqual(LEqual<'a>),
    LLess(LLess<'a>),
    LNot(LNot<'a>),
    MethodInvocation(MethodInvocation<'a>),
    ReferenceTypeOp(ReferenceTypeOp<'a>),
    Release(Release<'a>),
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
const LNOT_OP: u8 = 0x92;
const LEQUAL_OP: u8 = 0x93;
const LLESS_OP: u8 = 0x95;
const TO_BUFFER_OP: u8 = 0x96;
const TO_HEX_STRING_OP: u8 = 0x98;

const EXT_OP_PREFIX: u8 = 0x5B;

const ACQUIRE_OP: u8 = 0x23;
const RELEASE_OP: u8 = 0x27;

impl<'a> Expression<'a> {
    pub(in crate::parser::ast) fn parse(
        stream: &mut Stream<'a>,
        context: &mut Context,
    ) -> Result<Self> {
        match Expression::parse_opt(stream, context)? {
            Some(expression) => Ok(expression),
            None => Err(Error::unexpected_byte(
                stream.next().unwrap(),
                stream.offset() - 1,
                "Expression",
            )),
        }
    }

    pub(in crate::parser::ast) fn parse_opt(
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
            LEQUAL_OP => LEqual::parse(stream, context).map(|lequal| Expression::LEqual(lequal)),
            LLESS_OP => LLess::parse(stream, context).map(|lless| Expression::LLess(lless)),
            LNOT_OP => LNot::parse(stream, context).map(|lnot| Expression::LNot(lnot)),
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
            EXT_OP_PREFIX => match_next!(stream, "Extended Expression",
                ACQUIRE_OP => Acquire::parse(stream, context).map(|acquire| Expression::Acquire(acquire)),
                RELEASE_OP => Release::parse(stream, context).map(|release| Expression::Release(release)),
            ),
            _ => {
                stream.prev();
                MethodInvocation::parse(stream, context)
                    .map(|method_invocation| Expression::MethodInvocation(method_invocation))
            }
        }.map(|expression| Some(expression))
    }
}

impl<'a> core::fmt::Display for Expression<'a> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Expression::Acquire(acquire) => acquire.fmt(f),
            Expression::Increment(increment) => increment.fmt(f),
            Expression::LEqual(lequal) => lequal.fmt(f),
            Expression::LLess(lless) => lless.fmt(f),
            Expression::LNot(lnot) => lnot.fmt(f),
            Expression::MethodInvocation(method_invocation) => method_invocation.fmt(f),
            Expression::ReferenceTypeOp(reference_type_op) => reference_type_op.fmt(f),
            Expression::Release(release) => release.fmt(f),
            Expression::ShiftLeft(shift_left) => shift_left.fmt(f),
            Expression::SizeOf(size_of) => size_of.fmt(f),
            Expression::Store(store) => store.fmt(f),
            Expression::Subtract(subtract) => subtract.fmt(f),
            Expression::ToBuffer(to_buffer) => to_buffer.fmt(f),
            Expression::ToHexString(to_hex_string) => to_hex_string.fmt(f),
        }
    }
}
