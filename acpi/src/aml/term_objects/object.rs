use super::namespace_modifier_objects::namespace_modifier_object;
use crate::{
    aml::{Context, Result, Stream},
    namespace::Namespace,
};

pub(in crate::aml) fn parse(
    stream: &mut Stream,
    namespace: &mut Namespace,
    context: &Context,
) -> Result<()> {
    namespace_modifier_object::parse(stream, namespace, context)
}
