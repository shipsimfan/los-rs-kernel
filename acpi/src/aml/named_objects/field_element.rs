use super::NamedField;
use crate::aml::{ASTNode, ByteStream, Result};

pub(super) enum FieldElement {
    NamedField(NamedField),
}

impl FieldElement {
    pub(super) fn parse(stream: &mut ByteStream) -> Result<Self> {
        NamedField::parse(stream).map(|named_field| FieldElement::NamedField(named_field))
    }
}

impl ASTNode for FieldElement {
    fn display(&self, f: &mut core::fmt::Formatter, depth: usize) -> core::fmt::Result {
        todo!()
    }
}
