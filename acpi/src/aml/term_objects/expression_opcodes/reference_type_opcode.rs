use super::{DerefOf, Index, RefOf};
use crate::aml::{impl_core_display, peek, Display, Result, Stream};

pub(in crate::aml) enum ReferenceTypeOpcode {
    DerefOf(DerefOf),
    Index(Index),
    RefOf(RefOf),
}

const REF_OF_OP: u8 = 0x71;
const DEREF_OF_OP: u8 = 0x83;
const INDEX_OP: u8 = 0x88;

impl ReferenceTypeOpcode {
    pub(in crate::aml) fn parse(stream: &mut Stream) -> Result<Option<Self>> {
        match peek!(stream) {
            REF_OF_OP => {
                stream.next();
                RefOf::parse(stream).map(|ref_of| Some(ReferenceTypeOpcode::RefOf(ref_of)))
            }
            DEREF_OF_OP => {
                stream.next();
                DerefOf::parse(stream).map(|deref_of| Some(ReferenceTypeOpcode::DerefOf(deref_of)))
            }
            INDEX_OP => {
                stream.next();
                Index::parse(stream).map(|index| Some(ReferenceTypeOpcode::Index(index)))
            }
            _ => Ok(None),
        }
    }
}

impl Display for ReferenceTypeOpcode {
    fn display(&self, f: &mut core::fmt::Formatter, depth: usize, last: bool) -> core::fmt::Result {
        match self {
            ReferenceTypeOpcode::DerefOf(deref_of) => deref_of.display(f, depth, last),
            ReferenceTypeOpcode::Index(index) => index.display(f, depth, last),
            ReferenceTypeOpcode::RefOf(ref_of) => ref_of.display(f, depth, last),
        }
    }
}

impl_core_display!(ReferenceTypeOpcode);
