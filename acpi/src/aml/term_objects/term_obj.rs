use super::{Object, StatementOpcode};
use crate::aml::{impl_core_display, Display, Result, Stream};

pub(super) enum TermObj {
    Object(Object),
    StatementOpcode(StatementOpcode),
}

impl TermObj {
    pub(super) fn parse(stream: &mut Stream) -> Result<Self> {
        match StatementOpcode::parse(stream)? {
            Some(statement_opcode) => Ok(TermObj::StatementOpcode(statement_opcode)),
            None => Object::parse(stream).map(|object| TermObj::Object(object)),
        }
    }
}

impl Display for TermObj {
    fn display(&self, f: &mut core::fmt::Formatter, depth: usize, last: bool) -> core::fmt::Result {
        match self {
            TermObj::Object(object) => object.display(f, depth, last),
            TermObj::StatementOpcode(statement_opcode) => statement_opcode.display(f, depth, last),
        }
    }
}

impl_core_display!(TermObj);
