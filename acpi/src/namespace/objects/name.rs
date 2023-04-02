use crate::{
    interpreter::DataObject,
    namespace::{display_name, display_prefix, impl_core_display, Display, Node},
};
use alloc::rc::{Rc, Weak};
use core::cell::RefCell;

pub(crate) struct Name {
    parent: Option<Weak<RefCell<dyn Node>>>,
    name: [u8; 4],
    data_object: DataObject,
}

impl Name {
    pub(crate) fn new(
        parent: Option<&Rc<RefCell<dyn Node>>>,
        name: [u8; 4],
        data_object: DataObject,
    ) -> Rc<RefCell<dyn Node>> {
        Rc::new(RefCell::new(Name {
            parent: parent.map(|parent| Rc::downgrade(parent)),
            name,
            data_object,
        }))
    }
}

impl Node for Name {
    fn name(&self) -> Option<[u8; 4]> {
        Some(self.name)
    }

    fn parent(&self) -> Option<Rc<RefCell<dyn Node>>> {
        self.parent.as_ref().map(|parent| parent.upgrade().unwrap())
    }
}

impl Display for Name {
    fn display(&self, f: &mut core::fmt::Formatter, depth: usize, _: bool) -> core::fmt::Result {
        display_prefix!(f, depth);
        write!(f, "Name (")?;
        display_name!(f, self.name);
        writeln!(f, ", {})", self.data_object)
    }
}

impl_core_display!(Name);
