use crate::namespace::Namespace;

mod context;
mod error;
mod macros;
mod name_objects;
mod package_length;
mod stream;
mod term_objects;

pub(self) use context::Context;
pub(self) use macros::*;
pub(self) use stream::Stream;

pub(super) use error::Error;

pub(super) type Result<T> = core::result::Result<T, Error>;

pub(super) fn parse_definition_block(
    definition_block: &[u8],
    namespace: &mut Namespace,
) -> Result<()> {
    let mut stream = Stream::new(definition_block, 0);
    let context = Context::new();

    term_objects::term_list::parse(&mut stream, namespace, &context)
}
