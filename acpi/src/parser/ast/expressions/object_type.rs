use super::{ref_of::RefOf, DerefOf, Index};
use crate::parser::{next, Context, Result, SimpleName, Stream};

pub(crate) enum ObjectType<'a> {
    DebugObj,
    DerefOf(DerefOf<'a>),
    Index(Index<'a>),
    RefOf(RefOf<'a>),
    SimpleName(SimpleName),
}

const REF_OF_OP: u8 = 0x71;
const DEREF_OF_OP: u8 = 0x83;
const INDEX_OP: u8 = 0x88;

const EXT_OP_PREFIX: u8 = 0x5B;

const DEBUG_OP: u8 = 0x31;

impl<'a> ObjectType<'a> {
    pub(super) fn parse(stream: &mut Stream<'a>, context: &mut Context) -> Result<Self> {
        match next!(stream, "Object Type") {
            DEREF_OF_OP => {
                DerefOf::parse(stream, context).map(|deref_of| ObjectType::DerefOf(deref_of))
            }
            INDEX_OP => Index::parse(stream, context).map(|index| ObjectType::Index(index)),
            REF_OF_OP => RefOf::parse(stream, context).map(|ref_of| ObjectType::RefOf(ref_of)),
            EXT_OP_PREFIX => match next!(stream, "Extended Object Type") {
                DEBUG_OP => Ok(ObjectType::DebugObj),
                _ => {
                    stream.prev();
                    stream.prev();
                    SimpleName::parse(stream).map(|simple_name| ObjectType::SimpleName(simple_name))
                }
            },
            _ => {
                stream.prev();
                SimpleName::parse(stream).map(|simple_name| ObjectType::SimpleName(simple_name))
            }
        }
    }
}

impl<'a> core::fmt::Display for ObjectType<'a> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "ObjectType (")?;

        match self {
            ObjectType::DebugObj => write!(f, "Debug"),
            ObjectType::DerefOf(deref_of) => deref_of.fmt(f),
            ObjectType::Index(index) => index.fmt(f),
            ObjectType::RefOf(ref_of) => ref_of.fmt(f),
            ObjectType::SimpleName(simple_name) => simple_name.fmt(f),
        }?;

        write!(f, ")")
    }
}
