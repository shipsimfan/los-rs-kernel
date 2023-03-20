use super::FieldElement;
use crate::aml::{ASTNode, ByteStream, Result};
use alloc::vec::Vec;

pub(super) struct FieldList {
    list: Vec<FieldElement>,
}

impl FieldList {
    pub(super) fn parse(stream: &mut ByteStream) -> Result<Self> {
        let mut list = Vec::new();
        while stream.peek().is_some() {
            list.push(FieldElement::parse(stream)?);
        }
        Ok(FieldList { list })
    }
}

impl ASTNode for FieldList {
    fn display(&self, f: &mut core::fmt::Formatter, depth: usize) -> core::fmt::Result {
        todo!()
    }
}

impl core::fmt::Display for FieldList {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.display(f, 0)
    }
}
