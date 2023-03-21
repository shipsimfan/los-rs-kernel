use super::object;
use crate::{
    aml::{Context, Result, Stream},
    namespace::Namespace,
};

pub(in crate::aml) fn parse(
    stream: &mut Stream,
    namespace: &mut Namespace,
    context: &Context,
) -> Result<()> {
    object::parse(stream, namespace, context)
}
