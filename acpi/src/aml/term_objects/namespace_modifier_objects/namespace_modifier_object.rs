use super::Scope;
use crate::aml::{impl_core_display, peek, Display, Result, Stream};

pub(in crate::aml::term_objects) enum NamespaceModifierObject {
    Scope(Scope),
}

const SCOPE_OP: u8 = 0x10;

impl NamespaceModifierObject {
    pub(in crate::aml::term_objects) fn parse(stream: &mut Stream) -> Result<Option<Self>> {
        match peek!(stream) {
            SCOPE_OP => {
                stream.next();
                Scope::parse(stream).map(|scope| Some(NamespaceModifierObject::Scope(scope)))
            }
            _ => Ok(None),
        }
    }
}

impl Display for NamespaceModifierObject {
    fn display(&self, f: &mut core::fmt::Formatter, depth: usize) -> core::fmt::Result {
        match self {
            NamespaceModifierObject::Scope(scope) => scope.display(f, depth),
        }
    }
}

impl_core_display!(NamespaceModifierObject);
