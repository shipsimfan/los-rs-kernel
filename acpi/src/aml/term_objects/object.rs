use super::{named_objects::named_object, namespace_modifier_objects::namespace_modifier_object};
use crate::{
    aml::{Context, Result, Stream},
    namespace::Namespace,
};

pub(in crate::aml) fn parse(
    stream: &mut Stream,
    namespace: &mut Namespace,
    context: &Context,
) -> Result<()> {
    if !named_object::parse(stream, namespace, context)? {
        namespace_modifier_object::parse(stream, namespace, context)
    } else {
        Ok(())
    }
}
