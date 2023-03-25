use super::{
    notify::Notify, Break, BreakPoint, Continue, Fatal, If, NoOp, Release, Reset, Return, Signal,
    Sleep, Stall, While,
};
use crate::aml::{impl_core_display, peek, peek_ahead, Display, Result, Stream};

pub(in crate::aml::term_objects) enum StatementOpcode {
    Break(Break),
    BreakPoint(BreakPoint),
    Continue(Continue),
    Fatal(Fatal),
    If(If),
    NoOp(NoOp),
    Notify(Notify),
    Release(Release),
    Reset(Reset),
    Return(Return),
    Signal(Signal),
    Sleep(Sleep),
    Stall(Stall),
    While(While),
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

const STALL_OP: u8 = 0x21;
const SLEEP_OP: u8 = 0x22;
const SIGNAL_OP: u8 = 0x24;
const RESET_OP: u8 = 0x26;
const RELEASE_OP: u8 = 0x27;
const FATAL_OP: u8 = 0x32;

impl StatementOpcode {
    pub(in crate::aml::term_objects) fn parse(stream: &mut Stream) -> Result<Option<Self>> {
        match peek!(stream) {
            NOTIFY_OP => {
                stream.next();
                Notify::parse(stream).map(|notify| Some(StatementOpcode::Notify(notify)))
            }
            WHILE_OP => {
                stream.next();
                While::parse(stream).map(|r#while| Some(StatementOpcode::While(r#while)))
            }
            CONTINUE_OP => {
                stream.next();
                Continue::parse(stream)
                    .map(|r#continue| Some(StatementOpcode::Continue(r#continue)))
            }
            IF_OP => {
                stream.next();
                If::parse(stream).map(|r#if| Some(StatementOpcode::If(r#if)))
            }
            NO_OP_OP => {
                stream.next();
                NoOp::parse(stream).map(|no_op| Some(StatementOpcode::NoOp(no_op)))
            }
            RETURN_OP => {
                stream.next();
                Return::parse(stream).map(|r#return| Some(StatementOpcode::Return(r#return)))
            }
            BREAK_OP => {
                stream.next();
                Break::parse(stream).map(|r#break| Some(StatementOpcode::Break(r#break)))
            }
            BREAK_POINT_OP => {
                stream.next();
                BreakPoint::parse(stream)
                    .map(|break_point| Some(StatementOpcode::BreakPoint(break_point)))
            }
            EXT_OP_PREFIX => match peek_ahead!(stream) {
                STALL_OP => {
                    stream.next();
                    stream.next();
                    Stall::parse(stream).map(|stall| Some(StatementOpcode::Stall(stall)))
                }
                SLEEP_OP => {
                    stream.next();
                    stream.next();
                    Sleep::parse(stream).map(|sleep| Some(StatementOpcode::Sleep(sleep)))
                }
                SIGNAL_OP => {
                    stream.next();
                    stream.next();
                    Signal::parse(stream).map(|signal| Some(StatementOpcode::Signal(signal)))
                }
                RESET_OP => {
                    stream.next();
                    stream.next();
                    Reset::parse(stream).map(|reset| Some(StatementOpcode::Reset(reset)))
                }
                RELEASE_OP => {
                    stream.next();
                    stream.next();
                    Release::parse(stream).map(|release| Some(StatementOpcode::Release(release)))
                }
                FATAL_OP => {
                    stream.next();
                    stream.next();
                    Fatal::parse(stream).map(|fatal| Some(StatementOpcode::Fatal(fatal)))
                }
                _ => Ok(None),
            },
            _ => Ok(None),
        }
    }
}

impl Display for StatementOpcode {
    fn display(&self, f: &mut core::fmt::Formatter, depth: usize, last: bool) -> core::fmt::Result {
        match self {
            StatementOpcode::Break(r#break) => r#break.display(f, depth, last),
            StatementOpcode::BreakPoint(break_point) => break_point.display(f, depth, last),
            StatementOpcode::Continue(r#continue) => r#continue.display(f, depth, last),
            StatementOpcode::Fatal(fatal) => fatal.display(f, depth, last),
            StatementOpcode::If(r#if) => r#if.display(f, depth, last),
            StatementOpcode::NoOp(no_op) => no_op.display(f, depth, last),
            StatementOpcode::Notify(notify) => notify.display(f, depth, last),
            StatementOpcode::Release(release) => release.display(f, depth, last),
            StatementOpcode::Reset(reset) => reset.display(f, depth, last),
            StatementOpcode::Return(r#return) => r#return.display(f, depth, last),
            StatementOpcode::Signal(signal) => signal.display(f, depth, last),
            StatementOpcode::Sleep(sleep) => sleep.display(f, depth, last),
            StatementOpcode::Stall(stall) => stall.display(f, depth, last),
            StatementOpcode::While(r#while) => r#while.display(f, depth, last),
        }
    }
}

impl_core_display!(StatementOpcode);
