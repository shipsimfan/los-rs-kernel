use super::{Children, Node, Scope};
use alloc::rc::Rc;
use core::cell::RefCell;

pub(crate) struct Namespace {
    root: Rc<RefCell<dyn Node>>,
}

impl Namespace {
    pub(crate) fn new() -> Self {
        let root = Scope::new(None, None);

        let mut root_scope_ref = root.borrow_mut();

        let root_scope = root_scope_ref.as_any_mut().downcast_mut::<Scope>().unwrap();
        root_scope.add_child(Scope::new(Some(&root), Some([b'_', b'G', b'P', b'E'])));
        root_scope.add_child(Scope::new(Some(&root), Some([b'_', b'P', b'R', b'_'])));
        root_scope.add_child(Scope::new(Some(&root), Some([b'_', b'S', b'B', b'_'])));
        root_scope.add_child(Scope::new(Some(&root), Some([b'_', b'S', b'I', b'_'])));
        root_scope.add_child(Scope::new(Some(&root), Some([b'_', b'T', b'Z', b'_'])));

        drop(root_scope_ref);

        Namespace { root }
    }
}

impl core::fmt::Display for Namespace {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.root.borrow().fmt(f)
    }
}
