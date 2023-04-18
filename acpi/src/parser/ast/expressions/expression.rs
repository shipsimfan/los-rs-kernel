use super::{
    acquire::Acquire, size_of::SizeOf, Add, And, Concat, ConcatRes, CondRefOf, CopyObject,
    Decrement, Divide, FindSetLeftBit, FindSetRightBit, FromBCD, Increment, LAnd, LEqual, LGreater,
    LGreaterEqual, LLess, LLessEqual, LNot, LNotEqual, LOr, LoadTable, Match, MethodInvocation,
    Mid, Mod, Multiply, NAnd, NOr, Not, Or, ReferenceTypeOp, ShiftLeft, ShiftRight, Store,
    Subtract, ToBCD, ToBuffer, ToDecimalString, ToHexString, ToInteger, ToString, Xor,
};
use crate::parser::{match_next, next, Context, Error, Result, Stream};

pub(crate) enum Expression<'a> {
    Acquire(Acquire<'a>),
    Add(Add<'a>),
    And(And<'a>),
    Concat(Concat<'a>),
    ConcatRes(ConcatRes<'a>),
    CondRefOf(CondRefOf<'a>),
    CopyObject(CopyObject<'a>),
    Decrement(Decrement<'a>),
    Divide(Divide<'a>),
    FindSetLeftBit(FindSetLeftBit<'a>),
    FindSetRightBit(FindSetRightBit<'a>),
    FromBCD(FromBCD<'a>),
    Increment(Increment<'a>),
    LAnd(LAnd<'a>),
    LEqual(LEqual<'a>),
    LGreater(LGreater<'a>),
    LGreaterEqual(LGreaterEqual<'a>),
    LLess(LLess<'a>),
    LLessEqual(LLessEqual<'a>),
    LNot(LNot<'a>),
    LNotEqual(LNotEqual<'a>),
    LoadTable(LoadTable<'a>),
    LOr(LOr<'a>),
    Match(Match<'a>),
    MethodInvocation(MethodInvocation<'a>),
    Mid(Mid<'a>),
    Mod(Mod<'a>),
    Multiply(Multiply<'a>),
    NAnd(NAnd<'a>),
    NOr(NOr<'a>),
    Not(Not<'a>),
    Or(Or<'a>),
    ReferenceTypeOp(ReferenceTypeOp<'a>),
    ShiftLeft(ShiftLeft<'a>),
    ShiftRight(ShiftRight<'a>),
    SizeOf(SizeOf<'a>),
    Store(Store<'a>),
    Subtract(Subtract<'a>),
    ToBCD(ToBCD<'a>),
    ToBuffer(ToBuffer<'a>),
    ToDecimalString(ToDecimalString<'a>),
    ToHexString(ToHexString<'a>),
    ToInteger(ToInteger<'a>),
    ToString(ToString<'a>),
    Xor(Xor<'a>),
}

const STORE_OP: u8 = 0x70;
const ADD_OP: u8 = 0x72;
const CONCAT_OP: u8 = 0x73;
const SUBTRACT_OP: u8 = 0x74;
const INCREMENT_OP: u8 = 0x75;
const DECREMENT_OP: u8 = 0x76;
const MULTIPLY_OP: u8 = 0x77;
const DIVIDE_OP: u8 = 0x78;
const SHIFT_LEFT_OP: u8 = 0x79;
const SHIFT_RIGHT_OP: u8 = 0x7A;
const AND_OP: u8 = 0x7B;
const NAND_OP: u8 = 0x7C;
const OR_OP: u8 = 0x7D;
const NOR_OP: u8 = 0x7E;
const XOR_OP: u8 = 0x7F;
const NOT_OP: u8 = 0x80;
const FIND_SET_LEFT_BIT_OP: u8 = 0x81;
const FIND_SET_RIGHT_BIT_OP: u8 = 0x82;
const CONCAT_RES_OP: u8 = 0x84;
const MOD_OP: u8 = 0x85;
const SIZE_OF_OP: u8 = 0x87;
const MATCH_OP: u8 = 0x89;
const LAND_OP: u8 = 0x90;
const LOR_OP: u8 = 0x91;
const LNOT_OP: u8 = 0x92;
const LEQUAL_OP: u8 = 0x93;
const LGREATER_OP: u8 = 0x94;
const LLESS_OP: u8 = 0x95;
const TO_BUFFER_OP: u8 = 0x96;
const TO_DECIMAL_STRING_OP: u8 = 0x97;
const TO_HEX_STRING_OP: u8 = 0x98;
const TO_INTEGER_OP: u8 = 0x99;
const TO_STRING_OP: u8 = 0x9C;
const COPY_OBJECT_OP: u8 = 0x9D;
const MID_OP: u8 = 0x9E;

const EXT_OP_PREFIX: u8 = 0x5B;

const COND_REF_OF_OP: u8 = 0x12;
const LOAD_TABLE_OP: u8 = 0x1F;
const ACQUIRE_OP: u8 = 0x23;
const FROM_BCD: u8 = 0x28;
const TO_BCD_OP: u8 = 0x29;

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
            COPY_OBJECT_OP => CopyObject::parse(stream, context).map(|copy_object| Expression::CopyObject(copy_object)),
            DECREMENT_OP => Decrement::parse(stream, context).map(|decrement| Expression::Decrement(decrement)),
            DIVIDE_OP => Divide::parse(stream, context).map(|divide| Expression::Divide(divide)),
            FIND_SET_LEFT_BIT_OP => FindSetLeftBit::parse(stream, context).map(|find_set_left_bit| Expression::FindSetLeftBit(find_set_left_bit)),
            FIND_SET_RIGHT_BIT_OP => FindSetRightBit::parse(stream, context).map(|find_set_left_bit| Expression::FindSetRightBit(find_set_left_bit)),
            INCREMENT_OP => Increment::parse(stream, context).map(|increment| Expression::Increment(increment)),
            LAND_OP => LAnd::parse(stream, context).map(|land| Expression::LAnd(land)),
            LEQUAL_OP => LEqual::parse(stream, context).map(|lequal| Expression::LEqual(lequal)),
            LGREATER_OP => LGreater::parse(stream, context).map(|lgreater| Expression::LGreater(lgreater)),
            LLESS_OP => LLess::parse(stream, context).map(|lless| Expression::LLess(lless)),
            LNOT_OP => match next!(stream, "LNot") {
                LEQUAL_OP => LNotEqual::parse(stream, context).map(|lnot_equal| Expression::LNotEqual(lnot_equal)),
                LGREATER_OP => LLessEqual::parse(stream, context).map(|lless_equal| Expression::LLessEqual(lless_equal)),
                LLESS_OP => LGreaterEqual::parse(stream, context).map(|lgreater_equal| Expression::LGreaterEqual(lgreater_equal)),
                _ => {
                    stream.prev();
                    LNot::parse(stream, context).map(|lnot| Expression::LNot(lnot))
                }
            },
            LOR_OP => LOr::parse(stream, context).map(|lor| Expression::LOr(lor)),
            MATCH_OP => Match::parse(stream, context).map(|r#match| Expression::Match(r#match)),
            MID_OP => Mid::parse(stream, context).map(|mid| Expression::Mid(mid)),
            MOD_OP => Mod::parse(stream, context).map(|r#mod| Expression::Mod(r#mod)),
            MULTIPLY_OP => Multiply::parse(stream, context).map(|multiply| Expression::Multiply(multiply)),
            NAND_OP => NAnd::parse(stream, context).map(|nand| Expression::NAnd(nand)),
            NOR_OP => NOr::parse(stream, context).map(|nor| Expression::NOr(nor)),
            NOT_OP => Not::parse(stream, context).map(|not| Expression::Not(not)),
            OR_OP => Or::parse(stream, context).map(|or| Expression::Or(or)),
            SHIFT_LEFT_OP => ShiftLeft::parse(stream, context).map(|shift_left| Expression::ShiftLeft(shift_left)),
            SHIFT_RIGHT_OP => ShiftRight::parse(stream, context).map(|shift_right| Expression::ShiftRight(shift_right)),
            SIZE_OF_OP => SizeOf::parse(stream, context).map(|size_of| Expression::SizeOf(size_of)),
            STORE_OP => Store::parse(stream, context).map(|store| Expression::Store(store)),
            SUBTRACT_OP => Subtract::parse(stream, context).map(|subtract| Expression::Subtract(subtract)),
            TO_BUFFER_OP => ToBuffer::parse(stream, context).map(|to_buffer| Expression::ToBuffer(to_buffer)),
            TO_DECIMAL_STRING_OP => ToDecimalString::parse(stream, context).map(|to_decimal_string| Expression::ToDecimalString(to_decimal_string)),
            TO_HEX_STRING_OP => ToHexString::parse(stream, context).map(|to_hex_string| Expression::ToHexString(to_hex_string)),
            TO_INTEGER_OP => ToInteger::parse(stream, context).map(|to_integer| Expression::ToInteger(to_integer)),
            TO_STRING_OP => ToString::parse(stream, context).map(|to_string| Expression::ToString(to_string)),
            XOR_OP => Xor::parse(stream, context).map(|xor| Expression::Xor(xor)),
            EXT_OP_PREFIX => match_next!(stream, "Extended Expression",
                ACQUIRE_OP => Acquire::parse(stream, context).map(|acquire| Expression::Acquire(acquire)),
                COND_REF_OF_OP => CondRefOf::parse(stream, context).map(|cond_ref_of| Expression::CondRefOf(cond_ref_of)),
                FROM_BCD => FromBCD::parse(stream, context).map(|from_bcd| Expression::FromBCD(from_bcd)),
                LOAD_TABLE_OP => LoadTable::parse(stream, context).map(|load_table| Expression::LoadTable(load_table)),
                TO_BCD_OP => ToBCD::parse(stream, context).map(|to_bcd| Expression::ToBCD(to_bcd)),
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
            Expression::CondRefOf(cond_ref_of) => cond_ref_of.fmt(f),
            Expression::CopyObject(copy_object) => copy_object.fmt(f),
            Expression::Decrement(decrement) => decrement.fmt(f),
            Expression::Divide(divide) => divide.fmt(f),
            Expression::FindSetLeftBit(find_set_left_bit) => find_set_left_bit.fmt(f),
            Expression::FindSetRightBit(find_set_right_bit) => find_set_right_bit.fmt(f),
            Expression::FromBCD(from_bcd) => from_bcd.fmt(f),
            Expression::Increment(increment) => increment.fmt(f),
            Expression::LAnd(land) => land.fmt(f),
            Expression::LEqual(lequal) => lequal.fmt(f),
            Expression::LGreater(lgreator) => lgreator.fmt(f),
            Expression::LGreaterEqual(lgreater_equal) => lgreater_equal.fmt(f),
            Expression::LLess(lless) => lless.fmt(f),
            Expression::LLessEqual(lless_equal) => lless_equal.fmt(f),
            Expression::LNot(lnot) => lnot.fmt(f),
            Expression::LNotEqual(lnot_equal) => lnot_equal.fmt(f),
            Expression::LoadTable(load_table) => load_table.fmt(f),
            Expression::LOr(lor) => lor.fmt(f),
            Expression::Match(r#match) => r#match.fmt(f),
            Expression::MethodInvocation(method_invocation) => method_invocation.fmt(f),
            Expression::Mid(mid) => mid.fmt(f),
            Expression::Mod(r#mod) => r#mod.fmt(f),
            Expression::Multiply(multiply) => multiply.fmt(f),
            Expression::NAnd(nand) => nand.fmt(f),
            Expression::NOr(nor) => nor.fmt(f),
            Expression::Not(not) => not.fmt(f),
            Expression::Or(or) => or.fmt(f),
            Expression::ReferenceTypeOp(reference_type_op) => reference_type_op.fmt(f),
            Expression::ShiftLeft(shift_left) => shift_left.fmt(f),
            Expression::ShiftRight(shift_right) => shift_right.fmt(f),
            Expression::SizeOf(size_of) => size_of.fmt(f),
            Expression::Store(store) => store.fmt(f),
            Expression::Subtract(subtract) => subtract.fmt(f),
            Expression::ToBCD(to_bcd) => to_bcd.fmt(f),
            Expression::ToBuffer(to_buffer) => to_buffer.fmt(f),
            Expression::ToDecimalString(to_decimal_string) => to_decimal_string.fmt(f),
            Expression::ToHexString(to_hex_string) => to_hex_string.fmt(f),
            Expression::ToInteger(to_integer) => to_integer.fmt(f),
            Expression::ToString(to_string) => to_string.fmt(f),
            Expression::Xor(xor) => xor.fmt(f),
        }
    }
}
