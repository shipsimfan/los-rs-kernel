use super::{method::Method, Scope};
use crate::{
    impl_core_display,
    parser::{match_next, Context, Result, Stream},
    Display,
};

pub(crate) enum Term {
    Method(Method),
    Scope(Scope),
}

const SCOPE_OP: u8 = 0x10;
const METHOD_OP: u8 = 0x14;

impl Term {
    pub(super) fn parse(stream: &mut Stream, context: &mut Context) -> Result<Self> {
        match_next!(stream, "Term",
            METHOD_OP => Method::parse(stream, context).map(|method| Term::Method(method)),
            SCOPE_OP => Scope::parse(stream, context).map(|scope| Term::Scope(scope)),
        )
    }
}

impl Display for Term {
    fn display(&self, f: &mut core::fmt::Formatter, depth: usize, last: bool) -> core::fmt::Result {
        match self {
            Term::Method(method) => method.display(f, depth, last),
            Term::Scope(scope) => scope.display(f, depth, last),
        }
    }
}

impl_core_display!(Term);
