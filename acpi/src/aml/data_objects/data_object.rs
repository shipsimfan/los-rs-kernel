use super::ComputationalData;
use crate::aml::{ASTNode, ByteStream, Result};

pub(in crate::aml) enum DataObject {
    ComputationalData(ComputationalData),
}

impl DataObject {
    pub(in crate::aml) fn parse(stream: &mut ByteStream) -> Result<Self> {
        ComputationalData::parse(stream)
            .map(|computational_data| DataObject::ComputationalData(computational_data))
    }
}

impl ASTNode for DataObject {
    fn display(&self, f: &mut core::fmt::Formatter, depth: usize) -> core::fmt::Result {
        match self {
            DataObject::ComputationalData(computational_data) => {
                computational_data.display(f, depth)
            }
        }
    }
}
