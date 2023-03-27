use super::{ExpressionOpcode, Object, StatementOpcode};
use crate::aml::{impl_core_display, Display, Result, Stream};

pub(super) enum TermObj {
    Object(Object),
    StatementOpcode(StatementOpcode),
    ExpressionOpcode(ExpressionOpcode),
}

impl TermObj {
    pub(super) fn parse(stream: &mut Stream) -> Result<Self> {
        match StatementOpcode::parse(stream)? {
            Some(statement_opcode) => return Ok(TermObj::StatementOpcode(statement_opcode)),
            None => {}
        }

        match Object::parse(stream)? {
            Some(object) => Ok(TermObj::Object(object)),
            None => ExpressionOpcode::parse(stream)
                .map(|expression_opcode| TermObj::ExpressionOpcode(expression_opcode)),
        }
    }
}

impl Display for TermObj {
    fn display(&self, f: &mut core::fmt::Formatter, depth: usize, last: bool) -> core::fmt::Result {
        match self {
            TermObj::Object(object) => object.display(f, depth, last),
            TermObj::StatementOpcode(statement_opcode) => statement_opcode.display(f, depth, last),
            TermObj::ExpressionOpcode(expression_opcode) => {
                expression_opcode.display(f, depth, last)
            }
        }
    }
}

impl_core_display!(TermObj);
