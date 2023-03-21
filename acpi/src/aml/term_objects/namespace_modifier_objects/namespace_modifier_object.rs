use super::scope;
use crate::{
    aml::{match_next, Context, Result, Stream},
    namespace::Namespace,
};

const SCOPE_OP: u8 = 0x10;

pub(in crate::aml) fn parse(
    stream: &mut Stream,
    namespace: &mut Namespace,
    context: &Context,
) -> Result<()> {
    match_next!(stream,
        SCOPE_OP => scope::parse(stream, namespace, context)
    )
}
