use super::term_obj;
use crate::{
    aml::{Context, Result, Stream},
    namespace::Namespace,
};

pub(in crate::aml) fn parse(
    stream: &mut Stream,
    namespace: &mut Namespace,
    context: &Context,
) -> Result<()> {
    while stream.peek().is_some() {
        term_obj::parse(stream, namespace, context)?;
    }

    Ok(())
}
