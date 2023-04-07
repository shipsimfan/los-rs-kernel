use crate::namespace::{display_name, display_prefix, impl_core_display, Children, Display, Node};
use alloc::rc::{Rc, Weak};
use core::cell::RefCell;

pub(crate) struct ThermalZone<'a> {
    parent: Option<Weak<RefCell<Node<'a>>>>,
    name: [u8; 4],
    children: Children<'a>,
}

impl<'a> ThermalZone<'a> {
    pub(crate) fn new(parent: Option<&Rc<RefCell<Node<'a>>>>, name: [u8; 4]) -> Rc<RefCell<Node<'a>>> {
        Rc::new(RefCell::new(Node::ThermalZone(ThermalZone {
            parent: parent.map(|parent| Rc::downgrade(parent)),
            name,
            children: Children::new(),
        })))
    }

    pub(in crate::namespace) fn name(&self) -> [u8; 4] {
        self.name
    }

    pub(in crate::namespace) fn parent(&self) -> Option<Rc<RefCell<Node<'a>>>> {
        self.parent.as_ref().map(|parent| parent.upgrade().unwrap())
    }

    pub(in crate::namespace) fn children(&self) -> &Children<'a> {
        &self.children
    }

    pub(in crate::namespace) fn children_mut(&mut self) -> &mut Children<'a> {
        &mut self.children
    }
}

impl<'a> Display for ThermalZone<'a> {
    fn display(&self, f: &mut core::fmt::Formatter, depth: usize, last: bool) -> core::fmt::Result {
        display_prefix!(f, depth);
        write!(f, "ThermalZone (")?;
        display_name!(f, self.name());
        write!(f, ") ")?;
        self.children.display(f, depth, last)
    }
}

impl_core_display!(ThermalZone);
