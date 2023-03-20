use crate::aml::{ASTNode, ByteStream, DataObject, Result};

pub(in crate::aml) enum TermArg {
    DataObject(DataObject),
}

impl TermArg {
    pub(in crate::aml) fn parse(stream: &mut ByteStream) -> Result<Self> {
        DataObject::parse(stream).map(|data_object| TermArg::DataObject(data_object))
    }
}

impl ASTNode for TermArg {
    fn display(&self, f: &mut core::fmt::Formatter, depth: usize) -> core::fmt::Result {
        match self {
            TermArg::DataObject(data_object) => data_object.display(f, depth),
        }
    }
}
