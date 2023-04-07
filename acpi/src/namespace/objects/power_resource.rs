use crate::namespace::{display_name, display_prefix, impl_core_display, Children, Display, Node};
use alloc::rc::{Rc, Weak};
use core::cell::RefCell;

pub(crate) struct PowerResource<'a> {
    parent: Option<Weak<RefCell<Node<'a>>>>,
    name: [u8; 4],
    system_level: u8,
    resource_order: u16,
    children: Children<'a>,
}

impl<'a> PowerResource<'a> {
    pub(crate) fn new(
        parent: Option<&Rc<RefCell<Node<'a>>>>,
        name: [u8; 4],
        system_level: u8,
        resource_order: u16,
    ) -> Rc<RefCell<Node<'a>>> {
        Rc::new(RefCell::new(Node::PowerResource(PowerResource {
            parent: parent.map(|parent| Rc::downgrade(parent)),
            name,
            system_level,
            resource_order,
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

impl<'a> Display for PowerResource<'a> {
    fn display(&self, f: &mut core::fmt::Formatter, depth: usize, last: bool) -> core::fmt::Result {
        display_prefix!(f, depth);
        write!(f, "PowerResource (")?;
        display_name!(f, self.name());
        write!(f, ", {}, {}) ", self.system_level, self.resource_order)?;
        self.children.display(f, depth, last)
    }
}

impl_core_display!(PowerResource);
