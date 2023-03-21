use crate::{
    aml::{
        name_objects::name_string, next, term_objects::term_arg, unwrap_data_type, Context, Result,
        Stream,
    },
    namespace::{
        objects::{OperationRegion, RegionSpace},
        Namespace, Object,
    },
};

pub(super) fn parse(
    stream: &mut Stream,
    namespace: &mut Namespace,
    context: &Context,
) -> Result<()> {
    let (prefix, path, name) = name_string::parse(stream)?;

    let region_space = RegionSpace::from_u8(next!(stream));

    let offset = unwrap_data_type!(term_arg::parse(stream)?, Integer)?;
    let length = unwrap_data_type!(term_arg::parse(stream)?, Integer)?;

    let mut context = context.clone();
    context.move_down(prefix, &path, None, namespace)?;
    context
        .get_object(namespace)?
        .add_child(Object::OperationRegion(OperationRegion::new(
            name,
            offset,
            length,
            region_space,
        )))
}
