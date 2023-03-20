use crate::aml::{ASTNode, ByteStream, NamedObject, NamespaceModifierObject, Result};

pub(super) enum Object {
    NamespaceModifierObject(NamespaceModifierObject),
    NamedObject(NamedObject),
}

impl Object {
    pub(super) fn parse(stream: &mut ByteStream, c: u8) -> Result<Self> {
        match NamedObject::parse(stream, c)? {
            Some(object) => return Ok(Object::NamedObject(object)),
            None => {}
        }

        NamespaceModifierObject::parse(stream, c)
            .map(|object| Object::NamespaceModifierObject(object))
    }
}

impl ASTNode for Object {
    fn display(&self, f: &mut core::fmt::Formatter, depth: usize) -> core::fmt::Result {
        match self {
            Object::NamespaceModifierObject(object) => object.display(f, depth),
            Object::NamedObject(object) => object.display(f, depth),
        }
    }
}
