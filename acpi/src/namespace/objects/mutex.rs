use crate::namespace::{display_name, display_prefix, impl_core_display, Display, Node};
use alloc::rc::{Rc, Weak};
use core::cell::RefCell;

pub(crate) struct Mutex<'a> {
    parent: Option<Weak<RefCell<Node<'a>>>>,
    name: [u8; 4],
    sync_level: u8,
    // TODO: Implement mutex
}

impl<'a> Mutex<'a> {
    pub(crate) fn new(
        parent: Option<&Rc<RefCell<Node<'a>>>>,
        name: [u8; 4],
        sync_level: u8,
    ) -> Rc<RefCell<Node<'a>>> {
        Rc::new(RefCell::new(Node::Mutex(Mutex {
            parent: parent.map(|parent| Rc::downgrade(parent)),
            name,
            sync_level,
        })))
    }

    pub(in crate::namespace) fn name(&self) -> [u8; 4] {
        self.name
    }

    pub(in crate::namespace) fn parent(&self) -> Option<Rc<RefCell<Node<'a>>>> {
        self.parent.as_ref().map(|parent| parent.upgrade().unwrap())
    }
}

impl<'a> Display for Mutex<'a> {
    fn display(&self, f: &mut core::fmt::Formatter, depth: usize, _: bool) -> core::fmt::Result {
        display_prefix!(f, depth);
        write!(f, "Mutex (")?;
        display_name!(f, self.name);
        writeln!(f, ", {})", self.sync_level)
    }
}

impl_core_display!(Mutex);
