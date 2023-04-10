mod ast;
mod context;
mod error;
mod macros;
mod name_string;
mod pkg_length;
mod stream;

pub(self) use ast::AST;
pub(self) use context::Context;
pub(self) use error::Result;
pub(self) use macros::{match_next, next};
pub(self) use stream::Stream;

pub(crate) use error::Error;

pub(crate) fn parse_definition_block(
    definition_block: &[u8],
    logger: base::Logger,
    wide_integers: bool,
) -> Result<AST> {
    let context = Context::new(logger, wide_integers);
    AST::parse(definition_block, context)
}
