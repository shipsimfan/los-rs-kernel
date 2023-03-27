use super::{
    Acquire, DerefOf, Increment, Index, LLess, MethodInvocation, RefOf, Release, ShiftLeft, SizeOf,
    Store, Subtract, ToBuffer, ToHexString,
};
use crate::aml::{impl_core_display, match_next, peek, Display, Result, Stream};

pub(in crate::aml::term_objects) enum ExpressionOpcode {
    Acquire(Acquire),
    DerefOf(DerefOf),
    Increment(Increment),
    Index(Index),
    LLess(LLess),
    MethodInvocation(MethodInvocation),
    RefOf(RefOf),
    Release(Release),
    ShiftLeft(ShiftLeft),
    SizeOf(SizeOf),
    Store(Store),
    Subtract(Subtract),
    ToBuffer(ToBuffer),
    ToHexString(ToHexString),
}

const STORE_OP: u8 = 0x70;
const REF_OF_OP: u8 = 0x71;
const SUBTRACT_OP: u8 = 0x74;
const INCREMENT_OP: u8 = 0x75;
const SHIFT_LEFT_OP: u8 = 0x79;
const DEREF_OF_OP: u8 = 0x83;
const SIZE_OF_OP: u8 = 0x87;
const INDEX_OP: u8 = 0x88;
const LLESS_OP: u8 = 0x95;
const TO_BUFFER_OP: u8 = 0x96;
const TO_HEX_STRING_OP: u8 = 0x98;

const EXT_OP_PREFIX: u8 = 0x5B;

const ACQUIRE_OP: u8 = 0x23;
const RELEASE_OP: u8 = 0x27;

impl ExpressionOpcode {
    pub(in crate::aml::term_objects) fn parse(stream: &mut Stream) -> Result<Self> {
        match peek!(stream) {
            STORE_OP => {
                stream.next();
                Store::parse(stream).map(|store| ExpressionOpcode::Store(store))
            }
            REF_OF_OP => {
                stream.next();
                RefOf::parse(stream).map(|ref_of| ExpressionOpcode::RefOf(ref_of))
            }
            SUBTRACT_OP => {
                stream.next();
                Subtract::parse(stream).map(|subtract| ExpressionOpcode::Subtract(subtract))
            }
            INCREMENT_OP => {
                stream.next();
                Increment::parse(stream).map(|increment| ExpressionOpcode::Increment(increment))
            }
            SHIFT_LEFT_OP => {
                stream.next();
                ShiftLeft::parse(stream).map(|shift_left| ExpressionOpcode::ShiftLeft(shift_left))
            }
            DEREF_OF_OP => {
                stream.next();
                DerefOf::parse(stream).map(|deref_of| ExpressionOpcode::DerefOf(deref_of))
            }
            SIZE_OF_OP => {
                stream.next();
                SizeOf::parse(stream).map(|size_of| ExpressionOpcode::SizeOf(size_of))
            }
            INDEX_OP => {
                stream.next();
                Index::parse(stream).map(|index| ExpressionOpcode::Index(index))
            }
            LLESS_OP => {
                stream.next();
                LLess::parse(stream).map(|l_less| ExpressionOpcode::LLess(l_less))
            }
            TO_BUFFER_OP => {
                stream.next();
                ToBuffer::parse(stream).map(|to_buffer| ExpressionOpcode::ToBuffer(to_buffer))
            }
            TO_HEX_STRING_OP => {
                stream.next();
                ToHexString::parse(stream)
                    .map(|to_hex_string| ExpressionOpcode::ToHexString(to_hex_string))
            }
            EXT_OP_PREFIX => {
                stream.next();
                match_next!(stream,
                ACQUIRE_OP => Acquire::parse(stream).map(|acquire| ExpressionOpcode::Acquire(acquire))
                RELEASE_OP => Release::parse(stream).map(|release| ExpressionOpcode::Release(release))
                )
            }
            _ => MethodInvocation::parse(stream)
                .map(|method_invocation| ExpressionOpcode::MethodInvocation(method_invocation)),
        }
    }
}

impl Display for ExpressionOpcode {
    fn display(&self, f: &mut core::fmt::Formatter, depth: usize, last: bool) -> core::fmt::Result {
        match self {
            ExpressionOpcode::Acquire(acquire) => acquire.display(f, depth, last),
            ExpressionOpcode::DerefOf(deref_of) => deref_of.display(f, depth, last),
            ExpressionOpcode::Increment(increment) => increment.display(f, depth, last),
            ExpressionOpcode::Index(index) => index.display(f, depth, last),
            ExpressionOpcode::LLess(l_less) => l_less.display(f, depth, last),
            ExpressionOpcode::MethodInvocation(method_invocation) => {
                method_invocation.display(f, depth, last)
            }
            ExpressionOpcode::RefOf(ref_of) => ref_of.display(f, depth, last),
            ExpressionOpcode::Release(release) => release.display(f, depth, last),
            ExpressionOpcode::ShiftLeft(shift_left) => shift_left.display(f, depth, last),
            ExpressionOpcode::SizeOf(size_of) => size_of.display(f, depth, last),
            ExpressionOpcode::Store(store) => store.display(f, depth, last),
            ExpressionOpcode::Subtract(subtract) => subtract.display(f, depth, last),
            ExpressionOpcode::ToBuffer(to_buffer) => to_buffer.display(f, depth, last),
            ExpressionOpcode::ToHexString(to_hex_string) => to_hex_string.display(f, depth, last),
        }
    }
}

impl_core_display!(ExpressionOpcode);
