use super::{DataObject, ExpressionOpcode};
use crate::aml::{impl_core_display, ArgObj, Display, LocalObj, Result, Stream};

pub(super) enum TermArg {
    ArgObj(ArgObj),
    DataObject(DataObject),
    ExpressionOpcode(ExpressionOpcode),
    LocalObj(LocalObj),
}

impl TermArg {
    pub(super) fn parse(stream: &mut Stream) -> Result<Self> {
        match ArgObj::parse(stream)? {
            Some(arg_obj) => return Ok(TermArg::ArgObj(arg_obj)),
            None => {}
        }

        match LocalObj::parse(stream)? {
            Some(local_obj) => return Ok(TermArg::LocalObj(local_obj)),
            None => {}
        }

        match DataObject::parse(stream)? {
            Some(data_object) => Ok(TermArg::DataObject(data_object)),
            None => ExpressionOpcode::parse(stream)
                .map(|expression_opcode| TermArg::ExpressionOpcode(expression_opcode)),
        }
    }
}

impl Display for TermArg {
    fn display(&self, f: &mut core::fmt::Formatter, depth: usize, last: bool) -> core::fmt::Result {
        match self {
            TermArg::ArgObj(arg_obj) => arg_obj.display(f, depth, last),
            TermArg::DataObject(data_object) => data_object.display(f, depth, last),
            TermArg::ExpressionOpcode(expression_opcode) => {
                expression_opcode.display(f, depth, last)
            }
            TermArg::LocalObj(local_obj) => local_obj.display(f, depth, last),
        }
    }
}

impl_core_display!(TermArg);
