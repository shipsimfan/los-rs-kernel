use crate::{
    interpreter::DataObject,
    namespace::{display_name, display_prefix, impl_core_display, Display, Node},
};
use alloc::rc::{Rc, Weak};
use core::cell::RefCell;

pub(crate) struct Name<'a> {
    parent: Option<Weak<RefCell<Node<'a>>>>,
    name: [u8; 4],
    data_object: DataObject,
}

impl<'a> Name<'a> {
    pub(crate) fn new(
        parent: Option<&Rc<RefCell<Node<'a>>>>,
        name: [u8; 4],
        data_object: DataObject,
    ) -> Rc<RefCell<Node<'a>>> {
        Rc::new(RefCell::new(Node::Name(Name {
            parent: parent.map(|parent| Rc::downgrade(parent)),
            name,
            data_object,
        })))
    }

    pub(in crate::namespace) fn name(&self) -> [u8; 4] {
        self.name
    }

    pub(in crate::namespace) fn parent(&self) -> Option<Rc<RefCell<Node<'a>>>> {
        self.parent.as_ref().map(|parent| parent.upgrade().unwrap())
    }
}

impl<'a> Display for Name<'a> {
    fn display(&self, f: &mut core::fmt::Formatter, depth: usize, _: bool) -> core::fmt::Result {
        display_prefix!(f, depth);
        write!(f, "Name (")?;
        display_name!(f, self.name);
        writeln!(f, ", {})", self.data_object)
    }
}

impl_core_display!(Name);
