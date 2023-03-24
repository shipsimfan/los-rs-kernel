use super::{operation_region, OperationRegion, Scope};
use crate::{
    aml::Result,
    namespace::{impl_core_display, Display},
};

pub(crate) enum Object {
    Scope(Scope),
    OperationRegion(OperationRegion),
}

impl Object {
    pub(super) fn name(&self) -> Option<[u8; 4]> {
        match self {
            Object::Scope(scope) => scope.name(),
            Object::OperationRegion(operation_region) => operation_region.name(),
        }
    }

    pub(crate) fn get_child(&self, path: &[[u8; 4]]) -> Option<&Object> {
        if path.len() == 0 {
            return Some(self);
        }

        match self {
            Object::Scope(scope) => scope.get_child(path),
            Object::OperationRegion(_) => None,
        }
    }

    pub(crate) fn get_child_mut(&mut self, path: &[[u8; 4]]) -> Option<&mut Object> {
        if path.len() == 0 {
            return Some(self);
        }

        match self {
            Object::Scope(scope) => scope.get_child_mut(path),
            Object::OperationRegion(_) => None,
        }
    }

    pub(crate) fn add_child(&mut self, object: Object) -> Result<()> {
        match self {
            Object::Scope(scope) => scope.add_child(object),
            Object::OperationRegion(_) => Err(crate::aml::Error::AddChildNotScope),
        }
    }
}

impl Display for Object {
    fn display(&self, f: &mut core::fmt::Formatter, depth: usize, last: bool) -> core::fmt::Result {
        match self {
            Object::Scope(scope) => scope.display(f, depth),
            Object::OperationRegion(operation_region) => operation_region.display(f, depth),
        }
    }
}

impl_core_display!(Object);
