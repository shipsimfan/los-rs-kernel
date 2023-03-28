use super::Scope;
use crate::parser::{match_next, Result, Stream, SCOPE_OP};

pub(crate) enum Term<'a> {
    Scope(Scope<'a>),
}

impl<'a> Term<'a> {
    pub(super) fn parse(stream: &mut Stream<'a>) -> Result<Self> {
        match_next!(stream,
            SCOPE_OP => Scope::parse(stream).map(|scope| Term::Scope(scope))
        )
    }
}
