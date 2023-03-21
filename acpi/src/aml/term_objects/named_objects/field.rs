use crate::{
    aml::{name_objects::name_string, next, package_length, Context, Error, Result, Stream},
    namespace::{
        objects::{AccessType, Field, LockRule, UpdateRule},
        Namespace, Object,
    },
};

pub(in crate::aml) fn parse(
    stream: &mut Stream,
    namespace: &mut Namespace,
    context: &Context,
) -> Result<()> {
    let mut stream = package_length::parse_to_stream(stream)?;
    let (prefix, path, name) = name_string::parse(&mut stream)?;

    let mut context = context.clone();
    context.move_down(prefix, &path, name, namespace)?;

    let operation_region = match context.get_object(namespace)? {
        Object::OperationRegion(operation_region) => operation_region,
        _ => return Err(Error::FieldNotUnderOpRegion),
    };

    let field_flags = next!(stream);
    let access_type = AccessType::parse(field_flags & 0xF);
    let lock_rule = LockRule::parse(field_flags.wrapping_shr(4) & 1);
    let update_rule = UpdateRule::parse(field_flags.wrapping_shr(5) & 3);

    // TODO: Parse field units

    Ok(operation_region.add_field(Field::new(access_type, lock_rule, update_rule)))
}
