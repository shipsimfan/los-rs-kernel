use super::Scope;
use crate::{
    impl_core_display,
    parser::{match_next, Context, Result, Stream},
    Display,
};

pub(crate) enum Term {
    Scope(Scope),
}

const SCOPE_OP: u8 = 0x10;

impl Term {
    pub(super) fn parse(stream: &mut Stream, context: &mut Context) -> Result<Self> {
        match_next!(stream, "Term",
            SCOPE_OP => Scope::parse(stream, context).map(|scope| Term::Scope(scope)),
        )
    }
}

impl Display for Term {
    fn display(&self, f: &mut core::fmt::Formatter, depth: usize, last: bool) -> core::fmt::Result {
        match self {
            Term::Scope(scope) => scope.display(f, depth, last),
        }
    }
}

impl_core_display!(Term);
