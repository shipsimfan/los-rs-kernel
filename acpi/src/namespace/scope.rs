use super::{display_prefix, impl_core_display, Children, Display, Node};
use crate::namespace::display_name;
use alloc::rc::{Rc, Weak};
use core::cell::RefCell;

pub(crate) struct Scope<'a> {
    parent: Option<Weak<RefCell<Node<'a>>>>,
    name: Option<[u8; 4]>,
    children: Children<'a>,
}

impl<'a> Scope<'a> {
    pub(super) fn new_raw(parent: Option<&Rc<RefCell<Node<'a>>>>, name: Option<[u8; 4]>) -> Self {
        Scope {
            parent: parent.map(|parent| Rc::downgrade(parent)),
            name,
            children: Children::new(),
        }
    }

    pub(super) fn new(
        parent: Option<&Rc<RefCell<Node<'a>>>>,
        name: Option<[u8; 4]>,
    ) -> Rc<RefCell<Node<'a>>> {
        Rc::new(RefCell::new(Node::Scope(Self::new_raw(parent, name))))
    }

    pub(super) fn parent(&self) -> Option<Rc<RefCell<Node<'a>>>> {
        self.parent.as_ref().map(|parent| parent.upgrade().unwrap())
    }

    pub(super) fn name(&self) -> Option<[u8; 4]> {
        self.name
    }

    pub(super) fn children(&self) -> &Children<'a> {
        &self.children
    }

    pub(super) fn children_mut(&mut self) -> &mut Children<'a> {
        &mut self.children
    }
}

impl<'a> Display for Scope<'a> {
    fn display(&self, f: &mut core::fmt::Formatter, depth: usize, last: bool) -> core::fmt::Result {
        display_prefix!(f, depth);
        write!(f, "Scope (")?;

        match self.name {
            Some(name) => display_name!(f, name),
            None => write!(f, "\\")?,
        }

        write!(f, ") ")?;
        self.children.display(f, depth, last)
    }
}

impl_core_display!(Scope);
