use super::Scope;
use crate::aml::{impl_core_display, match_next, Display, Result, Stream};

pub(in crate::aml::term_objects) enum NamespaceModifierObject {
    Scope(Scope),
}

const SCOPE_OP: u8 = 0x10;

impl NamespaceModifierObject {
    pub(in crate::aml::term_objects) fn parse(stream: &mut Stream) -> Result<Self> {
        match_next!(stream,
            SCOPE_OP => Scope::parse(stream).map(|scope| NamespaceModifierObject::Scope(scope))
        )
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
