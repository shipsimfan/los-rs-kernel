use super::{Method, Scope};
use crate::parser::{match_next, Result, Stream, METHOD_OP, SCOPE_OP};

pub(crate) enum Term<'a> {
    Method(Method<'a>),
    Scope(Scope<'a>),
}

impl<'a> Term<'a> {
    pub(super) fn parse(stream: &mut Stream<'a>) -> Result<Self> {
        match_next!(stream,
            METHOD_OP => Method::parse(stream).map(|method| Term::Method(method))
            SCOPE_OP => Scope::parse(stream).map(|scope| Term::Scope(scope))
        )
    }
}
