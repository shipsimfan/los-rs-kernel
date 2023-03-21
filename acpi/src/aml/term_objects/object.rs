use super::NamespaceModifierObject;
use crate::aml::{impl_core_display, Display, Result, Stream};

pub(super) enum Object {
    NamespaceModifierObject(NamespaceModifierObject),
}

impl Object {
    pub(super) fn parse(stream: &mut Stream) -> Result<Self> {
        NamespaceModifierObject::parse(stream).map(|namespace_modifier_object| {
            Object::NamespaceModifierObject(namespace_modifier_object)
        })
    }
}

impl Display for Object {
    fn display(&self, f: &mut core::fmt::Formatter, depth: usize) -> core::fmt::Result {
        match self {
            Object::NamespaceModifierObject(namespace_modifier_object) => {
                namespace_modifier_object.display(f, depth)
            }
        }
    }
}

impl_core_display!(Object);
