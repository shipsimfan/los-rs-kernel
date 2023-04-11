use super::{ref_of::RefOf, DerefOf, Index};
use crate::parser::{next, Context, Result, Stream};

pub(crate) enum ReferenceTypeOp<'a> {
    DerefOf(DerefOf<'a>),
    Index(Index<'a>),
    RefOf(RefOf<'a>),
}

const REF_OP: u8 = 0x71;
const DEREF_OP: u8 = 0x83;
const INDEX_OP: u8 = 0x88;

impl<'a> ReferenceTypeOp<'a> {
    pub(in crate::parser) fn parse(
        stream: &mut Stream<'a>,
        context: &mut Context,
    ) -> Result<Option<Self>> {
        match next!(stream, "Reference Type Op") {
            DEREF_OP => {
                DerefOf::parse(stream, context).map(|deref_of| ReferenceTypeOp::DerefOf(deref_of))
            }
            INDEX_OP => Index::parse(stream, context).map(|index| ReferenceTypeOp::Index(index)),
            REF_OP => RefOf::parse(stream, context).map(|ref_of| ReferenceTypeOp::RefOf(ref_of)),
            _ => {
                stream.prev();
                return Ok(None);
            }
        }
        .map(|reference_type_op| Some(reference_type_op))
    }
}

impl<'a> core::fmt::Display for ReferenceTypeOp<'a> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            ReferenceTypeOp::DerefOf(deref_of) => deref_of.fmt(f),
            ReferenceTypeOp::Index(index) => index.fmt(f),
            ReferenceTypeOp::RefOf(ref_of) => ref_of.fmt(f),
        }
    }
}
