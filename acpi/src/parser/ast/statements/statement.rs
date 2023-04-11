use super::{Return, While};
use crate::{
    display_prefix, impl_core_display_lifetime,
    parser::{ast::Expression, next, Context, Result, Stream},
    Display,
};

pub(crate) enum Statement<'a> {
    Expression(Expression<'a>),
    Return(Return<'a>),
    While(While<'a>),
}

const WHILE_OP: u8 = 0xA2;
const RETURN_OP: u8 = 0xA4;

impl<'a> Statement<'a> {
    pub(in crate::parser::ast) fn parse(
        stream: &mut Stream<'a>,
        context: &mut Context,
    ) -> Result<Option<Self>> {
        if let Some(expression) = Expression::parse(stream, context)? {
            return Ok(Some(Statement::Expression(expression)));
        }

        match next!(stream, "Statement") {
            RETURN_OP => Return::parse(stream, context).map(|r#return| Statement::Return(r#return)),
            WHILE_OP => While::parse(stream, context).map(|r#while| Statement::While(r#while)),
            _ => {
                stream.prev();
                return Ok(None);
            }
        }
        .map(|statement| Some(statement))
    }
}

impl<'a> Display for Statement<'a> {
    fn display(&self, f: &mut core::fmt::Formatter, depth: usize, last: bool) -> core::fmt::Result {
        match self {
            Statement::Expression(expression) => {
                display_prefix!(f, depth);
                writeln!(f, "{}", expression)
            }
            Statement::Return(r#return) => r#return.display(f, depth, last),
            Statement::While(r#while) => r#while.display(f, depth, last),
        }
    }
}

impl_core_display_lifetime!(Statement);
