use crate::{
    aml::{
        name_objects::name_string, package_length, term_objects::term_list, Context, Result, Stream,
    },
    namespace::Namespace,
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

    term_list::parse(&mut stream, namespace, &context)
}
