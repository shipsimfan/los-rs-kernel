use super::{Field, Method, OpRegion, Scope};
use crate::parser::{
    match_next, Result, Stream, EXT_OP_PREFIX, FIELD_OP, METHOD_OP, OP_REGION_OP, SCOPE_OP,
};

pub(crate) enum Term<'a> {
    Field(Field<'a>),
    Method(Method<'a>),
    OpRegion(OpRegion),
    Scope(Scope<'a>),
}

impl<'a> Term<'a> {
    pub(super) fn parse(stream: &mut Stream<'a>) -> Result<Self> {
        match_next!(stream,
            METHOD_OP => Method::parse(stream).map(|method| Term::Method(method))
            SCOPE_OP => Scope::parse(stream).map(|scope| Term::Scope(scope))
            EXT_OP_PREFIX => match_next!(stream,
                FIELD_OP => Field::parse(stream).map(|field| Term::Field(field))
                OP_REGION_OP => OpRegion::parse(stream).map(|op_region| Term::OpRegion(op_region))
            )
        )
    }
}
