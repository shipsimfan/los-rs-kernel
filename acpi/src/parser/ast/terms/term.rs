use super::{Method, OpRegion, Scope};
use crate::{
    impl_core_display,
    parser::{match_next, Context, Result, Stream},
    Display,
};

pub(crate) enum Term<'a> {
    Method(Method<'a>),
    OpRegion(OpRegion<'a>),
    Scope(Scope<'a>),
}

const SCOPE_OP: u8 = 0x10;
const METHOD_OP: u8 = 0x14;

const EXT_OP_PREFIX: u8 = 0x5B;

const OP_REGION_OP: u8 = 0x80;

impl<'a> Term<'a> {
    pub(super) fn parse(stream: &mut Stream<'a>, context: &mut Context) -> Result<Self> {
        match_next!(stream, "Term",
            METHOD_OP => Method::parse(stream, context).map(|method| Term::Method(method)),
            SCOPE_OP => Scope::parse(stream, context).map(|scope| Term::Scope(scope)),
            EXT_OP_PREFIX => match_next!(stream, "Extended Term",
                OP_REGION_OP => OpRegion::parse(stream, context).map(|op_region| Term::OpRegion(op_region)),
            ),
        )
    }
}

impl<'a> Display for Term<'a> {
    fn display(&self, f: &mut core::fmt::Formatter, depth: usize, last: bool) -> core::fmt::Result {
        match self {
            Term::Method(method) => method.display(f, depth, last),
            Term::OpRegion(op_region) => op_region.display(f, depth, last),
            Term::Scope(scope) => scope.display(f, depth, last),
        }
    }
}

impl_core_display!(Term);
