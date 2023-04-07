use super::{Node, Scope};
use alloc::rc::Rc;
use core::cell::RefCell;

pub(crate) struct Namespace<'a> {
    root: Rc<RefCell<Node<'a>>>,
}

impl<'a> Namespace<'a> {
    pub(crate) fn new() -> Self {
        let root = Scope::new(None, None);

        let mut root_scope_ref = root.borrow_mut();

        let root_scope = root_scope_ref.children_mut().unwrap();
        root_scope.add_child(Scope::new(Some(&root), Some([b'_', b'G', b'P', b'E'])));
        root_scope.add_child(Scope::new(Some(&root), Some([b'_', b'P', b'R', b'_'])));
        root_scope.add_child(Scope::new(Some(&root), Some([b'_', b'S', b'B', b'_'])));
        root_scope.add_child(Scope::new(Some(&root), Some([b'_', b'S', b'I', b'_'])));
        root_scope.add_child(Scope::new(Some(&root), Some([b'_', b'T', b'Z', b'_'])));

        drop(root_scope_ref);

        Namespace { root }
    }

    pub(crate) fn root(&self) -> &Rc<RefCell<Node<'a>>> {
        &self.root
    }

    pub(crate) fn get_node(&self, path: &[[u8; 4]]) -> Option<Rc<RefCell<Node<'a>>>> {
        let mut node = self.root.clone();

        for name in path {
            let node_ref = node.borrow();

            let child = match node_ref.children() {
                Some(children) => match children.get_child(*name) {
                    Some(child) => child,
                    None => return None,
                },
                None => return None,
            };

            drop(node_ref);
            node = child;
        }

        Some(node)
    }
}

impl<'a> core::fmt::Display for Namespace<'a> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.root.borrow().fmt(f)
    }
}
