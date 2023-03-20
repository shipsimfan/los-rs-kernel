use super::DefScope;
use crate::aml::{ASTNode, ByteStream, Error, Result};

pub(in crate::aml) enum NamespaceModifierObject {
    DefScope(DefScope),
}

const SCOPE_OP: u8 = 0x10;

impl NamespaceModifierObject {
    pub(in crate::aml) fn parse(stream: &mut ByteStream, c: u8) -> Result<Self> {
        match c {
            SCOPE_OP => Ok(NamespaceModifierObject::DefScope(DefScope::parse(stream)?)),
            _ => Err(Error::UnexpectedByte(c)),
        }
    }
}

impl ASTNode for NamespaceModifierObject {
    fn display(&self, f: &mut core::fmt::Formatter, depth: usize) -> core::fmt::Result {
        match self {
            NamespaceModifierObject::DefScope(def_scope) => def_scope.display(f, depth),
        }
    }
}
