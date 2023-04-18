use super::{
    acquire::Acquire, size_of::SizeOf, Add, And, Concat, ConcatRes, Increment, LAnd, LEqual,
    LGreater, LLess, LNot, LOr, MethodInvocation, Or, ReferenceTypeOp, ShiftLeft, ShiftRight,
    Store, Subtract, ToBuffer, ToHexString,
};
use crate::parser::{match_next, next, Context, Error, Result, Stream};

pub(crate) enum Expression<'a> {
    Acquire(Acquire<'a>),
    Add(Add<'a>),
    And(And<'a>),
    Concat(Concat<'a>),
    ConcatRes(ConcatRes<'a>),
    Increment(Increment<'a>),
    LAnd(LAnd<'a>),
    LEqual(LEqual<'a>),
    LGreater(LGreater<'a>),
    LLess(LLess<'a>),
    LNot(LNot<'a>),
    LOr(LOr<'a>),
    MethodInvocation(MethodInvocation<'a>),
    Or(Or<'a>),
    ReferenceTypeOp(ReferenceTypeOp<'a>),
    ShiftLeft(ShiftLeft<'a>),
    ShiftRight(ShiftRight<'a>),
    SizeOf(SizeOf<'a>),
    Store(Store<'a>),
    Subtract(Subtract<'a>),
    ToBuffer(ToBuffer<'a>),
    ToHexString(ToHexString<'a>),
}

const STORE_OP: u8 = 0x70;
const ADD_OP: u8 = 0x72;
const CONCAT_OP: u8 = 0x73;
const SUBTRACT_OP: u8 = 0x74;
const INCREMENT_OP: u8 = 0x75;
const SHIFT_LEFT_OP: u8 = 0x79;
const SHIFT_RIGHT_OP: u8 = 0x7A;
const AND_OP: u8 = 0x7B;
const OR_OP: u8 = 0x7D;
const CONCAT_RES_OP: u8 = 0x84;
const SIZE_OF_OP: u8 = 0x87;
const LAND_OP: u8 = 0x90;
const LOR_OP: u8 = 0x91;
const LNOT_OP: u8 = 0x92;
const LEQUAL_OP: u8 = 0x93;
const LGREATER_OP: u8 = 0x94;
const LLESS_OP: u8 = 0x95;
const TO_BUFFER_OP: u8 = 0x96;
const TO_HEX_STRING_OP: u8 = 0x98;

const EXT_OP_PREFIX: u8 = 0x5B;

const ACQUIRE_OP: u8 = 0x23;

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
            ADD_OP => Add::parse(stream, context).map(|add| Expression::Add(add)),
            AND_OP => And::parse(stream, context).map(|and| Expression::And(and)),
            CONCAT_OP => Concat::parse(stream, context).map(|concat| Expression::Concat(concat)),
            CONCAT_RES_OP => ConcatRes::parse(stream, context).map(|concat_res| Expression::ConcatRes(concat_res)),
            INCREMENT_OP => {
                Increment::parse(stream, context).map(|increment| Expression::Increment(increment))
            }
            LAND_OP => LAnd::parse(stream, context).map(|land| Expression::LAnd(land)),
            LEQUAL_OP => LEqual::parse(stream, context).map(|lequal| Expression::LEqual(lequal)),
            LGREATER_OP => LGreater::parse(stream, context).map(|lgreater| Expression::LGreater(lgreater)),
            LLESS_OP => LLess::parse(stream, context).map(|lless| Expression::LLess(lless)),
            LNOT_OP => LNot::parse(stream, context).map(|lnot| Expression::LNot(lnot)),
            LOR_OP => LOr::parse(stream, context).map(|lor| Expression::LOr(lor)),
            OR_OP => Or::parse(stream, context).map(|or| Expression::Or(or)),
            SHIFT_LEFT_OP => ShiftLeft::parse(stream, context)
                .map(|shift_left| Expression::ShiftLeft(shift_left)),
            SHIFT_RIGHT_OP => ShiftRight::parse(stream, context).map(|shift_right| Expression::ShiftRight(shift_right)),
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
            Expression::Add(add) => add.fmt(f),
            Expression::And(and) => and.fmt(f),
            Expression::Concat(concat) => concat.fmt(f),
            Expression::ConcatRes(concat_res) => concat_res.fmt(f),
            Expression::Increment(increment) => increment.fmt(f),
            Expression::LAnd(land) => land.fmt(f),
            Expression::LEqual(lequal) => lequal.fmt(f),
            Expression::LGreater(lgreator) => lgreator.fmt(f),
            Expression::LLess(lless) => lless.fmt(f),
            Expression::LNot(lnot) => lnot.fmt(f),
            Expression::LOr(lor) => lor.fmt(f),
            Expression::MethodInvocation(method_invocation) => method_invocation.fmt(f),
            Expression::Or(or) => or.fmt(f),
            Expression::ReferenceTypeOp(reference_type_op) => reference_type_op.fmt(f),
            Expression::ShiftLeft(shift_left) => shift_left.fmt(f),
            Expression::ShiftRight(shift_right) => shift_right.fmt(f),
            Expression::SizeOf(size_of) => size_of.fmt(f),
            Expression::Store(store) => store.fmt(f),
            Expression::Subtract(subtract) => subtract.fmt(f),
            Expression::ToBuffer(to_buffer) => to_buffer.fmt(f),
            Expression::ToHexString(to_hex_string) => to_hex_string.fmt(f),
        }
    }
}
