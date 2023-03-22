use super::{Alias, Name, Scope};
use crate::aml::{impl_core_display, peek, Display, Result, Stream};

pub(in crate::aml::term_objects) enum NamespaceModifierObject {
    Alias(Alias),
    Name(Name),
    Scope(Scope),
}

const ALIAS_OP: u8 = 0x06;
const NAME_OP: u8 = 0x08;
const SCOPE_OP: u8 = 0x10;

impl NamespaceModifierObject {
    pub(in crate::aml::term_objects) fn parse(stream: &mut Stream) -> Result<Option<Self>> {
        match peek!(stream) {
            ALIAS_OP => {
                stream.next();
                Alias::parse(stream).map(|alias| Some(NamespaceModifierObject::Alias(alias)))
            }
            NAME_OP => {
                stream.next();
                Name::parse(stream).map(|name| Some(NamespaceModifierObject::Name(name)))
            }
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
            NamespaceModifierObject::Alias(alias) => alias.display(f, depth),
            NamespaceModifierObject::Name(name) => name.display(f, depth),
            NamespaceModifierObject::Scope(scope) => scope.display(f, depth),
        }
    }
}

impl_core_display!(NamespaceModifierObject);
