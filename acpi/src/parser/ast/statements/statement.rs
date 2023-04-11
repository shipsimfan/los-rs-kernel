use crate::{
    display_prefix, impl_core_display_lifetime,
    parser::{ast::Expression, next, Context, Result, Stream},
    Display,
};

pub(crate) enum Statement<'a> {
    Expression(Expression<'a>),
}

impl<'a> Statement<'a> {
    pub(in crate::parser::ast) fn parse(
        stream: &mut Stream<'a>,
        context: &mut Context,
    ) -> Result<Option<Self>> {
        if let Some(expression) = Expression::parse(stream, context)? {
            return Ok(Some(Statement::Expression(expression)));
        }

        match next!(stream, "Statement") {
            _ => {
                stream.prev();
                return Ok(None);
            }
        }
    }
}

impl<'a> Display for Statement<'a> {
    fn display(&self, f: &mut core::fmt::Formatter, depth: usize, _: bool) -> core::fmt::Result {
        match self {
            Statement::Expression(expression) => {
                display_prefix!(f, depth);
                writeln!(f, "{}", expression)
            }
        }
    }
}

impl_core_display_lifetime!(Statement);
