use super::{Break, BreakPoint, Continue, If, Notify, Return, While};
use crate::{
    display_prefix, impl_core_display_lifetime,
    parser::{ast::Expression, next, Context, Result, Stream},
    Display,
};

pub(crate) enum Statement<'a> {
    Break(Break),
    BreakPoint(BreakPoint),
    Continue(Continue),
    Expression(Expression<'a>),
    If(If<'a>),
    Notify(Notify<'a>),
    Return(Return<'a>),
    While(While<'a>),
}

const NOTIFY_OP: u8 = 0x86;
const CONTINUE_OP: u8 = 0x9F;
const IF_OP: u8 = 0xA0;
const WHILE_OP: u8 = 0xA2;
const RETURN_OP: u8 = 0xA4;
const BREAK_OP: u8 = 0xA5;
const BREAK_POINT_OP: u8 = 0xCC;

impl<'a> Statement<'a> {
    pub(in crate::parser::ast) fn parse(
        stream: &mut Stream<'a>,
        context: &mut Context,
    ) -> Result<Self> {
        match next!(stream, "Statement") {
            BREAK_OP => Ok(Statement::Break(Break)),
            BREAK_POINT_OP => Ok(Statement::BreakPoint(BreakPoint)),
            CONTINUE_OP => Ok(Statement::Continue(Continue)),
            IF_OP => If::parse(stream, context).map(|r#if| Statement::If(r#if)),
            NOTIFY_OP => Notify::parse(stream, context).map(|notify| Statement::Notify(notify)),
            RETURN_OP => Return::parse(stream, context).map(|r#return| Statement::Return(r#return)),
            WHILE_OP => While::parse(stream, context).map(|r#while| Statement::While(r#while)),
            _ => {
                stream.prev();
                Expression::parse(stream, context)
                    .map(|expression| Statement::Expression(expression))
            }
        }
    }
}

impl<'a> Display for Statement<'a> {
    fn display(
        &self,
        f: &mut core::fmt::Formatter,
        depth: usize,
        last: bool,
        newline: bool,
    ) -> core::fmt::Result {
        match self {
            Statement::Break(r#break) => r#break.display(f, depth, last, newline),
            Statement::BreakPoint(break_point) => break_point.display(f, depth, last, newline),
            Statement::Continue(r#continue) => r#continue.display(f, depth, last, newline),
            Statement::Expression(expression) => {
                display_prefix!(f, depth);
                write!(f, "{}", expression)?;

                if newline {
                    writeln!(f)
                } else {
                    Ok(())
                }
            }
            Statement::If(r#if) => r#if.display(f, depth, last, newline),
            Statement::Notify(notify) => notify.display(f, depth, last, newline),
            Statement::Return(r#return) => r#return.display(f, depth, last, newline),
            Statement::While(r#while) => r#while.display(f, depth, last, newline),
        }
    }
}

impl_core_display_lifetime!(Statement);
