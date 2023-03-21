use super::Scope;
use crate::{
    aml::Result,
    namespace::{impl_core_display, Display},
};

pub(crate) enum Object {
    Scope(Scope),
}

impl Object {
    pub(super) fn name(&self) -> Option<[u8; 4]> {
        match self {
            Object::Scope(scope) => scope.name(),
        }
    }

    pub(crate) fn get_child(&self, path: &[[u8; 4]]) -> Option<&Object> {
        if path.len() == 0 {
            return Some(self);
        }

        match self {
            Object::Scope(scope) => scope.get_child(path),
        }
    }

    pub(crate) fn get_child_mut(&mut self, path: &[[u8; 4]]) -> Option<&mut Object> {
        if path.len() == 0 {
            return Some(self);
        }

        match self {
            Object::Scope(scope) => scope.get_child_mut(path),
        }
    }

    pub(crate) fn add_child(&mut self, object: Object) -> Result<()> {
        match self {
            Object::Scope(scope) => scope.add_child(object),
        }
    }
}

impl Display for Object {
    fn display(&self, f: &mut core::fmt::Formatter, depth: usize) -> core::fmt::Result {
        match self {
            Object::Scope(scope) => scope.display(f, depth),
        }
    }
}

impl_core_display!(Object);
