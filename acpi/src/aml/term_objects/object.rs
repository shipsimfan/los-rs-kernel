use super::{NamedObject, NamespaceModifierObject};
use crate::aml::{impl_core_display, Display, Result, Stream};

pub(super) enum Object {
    NamespaceModifierObject(NamespaceModifierObject),
    NamedObject(NamedObject),
}

impl Object {
    pub(super) fn parse(stream: &mut Stream) -> Result<Self> {
        match NamespaceModifierObject::parse(stream)? {
            Some(namespace_modifier_object) => {
                Ok(Object::NamespaceModifierObject(namespace_modifier_object))
            }
            None => {
                NamedObject::parse(stream).map(|named_object| Object::NamedObject(named_object))
            }
        }
    }
}

impl Display for Object {
    fn display(&self, f: &mut core::fmt::Formatter, depth: usize) -> core::fmt::Result {
        match self {
            Object::NamespaceModifierObject(namespace_modifier_object) => {
                namespace_modifier_object.display(f, depth)
            }
            Object::NamedObject(named_object) => named_object.display(f, depth),
        }
    }
}

impl_core_display!(Object);
