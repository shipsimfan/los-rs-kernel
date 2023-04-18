use super::{Break, BreakPoint, Continue, Fatal, If, NoOp, Notify, Release, Reset, Return, While};
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
    Fatal(Fatal<'a>),
    If(If<'a>),
    NoOp(NoOp),
    Notify(Notify<'a>),
    Release(Release<'a>),
    Reset(Reset<'a>),
    Return(Return<'a>),
    While(While<'a>),
}

const NOTIFY_OP: u8 = 0x86;
const CONTINUE_OP: u8 = 0x9F;
const IF_OP: u8 = 0xA0;
const WHILE_OP: u8 = 0xA2;
const NO_OP_OP: u8 = 0xA3;
const RETURN_OP: u8 = 0xA4;
const BREAK_OP: u8 = 0xA5;
const BREAK_POINT_OP: u8 = 0xCC;

const EXT_OP_PREFIX: u8 = 0x5B;

const RESET_OP: u8 = 0x26;
const RELEASE_OP: u8 = 0x27;
const FATAL_OP: u8 = 0x32;

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
            NO_OP_OP => Ok(Statement::NoOp(NoOp)),
            NOTIFY_OP => Notify::parse(stream, context).map(|notify| Statement::Notify(notify)),
            RETURN_OP => Return::parse(stream, context).map(|r#return| Statement::Return(r#return)),
            WHILE_OP => While::parse(stream, context).map(|r#while| Statement::While(r#while)),
            EXT_OP_PREFIX => match next!(stream, "Extended Statement") {
                FATAL_OP => Fatal::parse(stream, context).map(|fatal| Statement::Fatal(fatal)),
                RELEASE_OP => {
                    Release::parse(stream, context).map(|release| Statement::Release(release))
                }
                RESET_OP => Reset::parse(stream, context).map(|reset| Statement::Reset(reset)),
                _ => {
                    stream.prev();
                    stream.prev();
                    Expression::parse(stream, context)
                        .map(|expression| Statement::Expression(expression))
                }
            },
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
            Statement::Fatal(fatal) => fatal.display(f, depth, last, newline),
            Statement::If(r#if) => r#if.display(f, depth, last, newline),
            Statement::NoOp(no_op) => no_op.display(f, depth, last, newline),
            Statement::Notify(notify) => notify.display(f, depth, last, newline),
            Statement::Release(release) => release.display(f, depth, last, newline),
            Statement::Reset(reset) => reset.display(f, depth, last, newline),
            Statement::Return(r#return) => r#return.display(f, depth, last, newline),
            Statement::While(r#while) => r#while.display(f, depth, last, newline),
        }
    }
}

impl_core_display_lifetime!(Statement);
