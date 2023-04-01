use crate::namespace::{display_name, display_prefix, impl_core_display, Display, Node};
use alloc::rc::{Rc, Weak};
use core::cell::RefCell;

pub(crate) struct Mutex {
    parent: Option<Weak<RefCell<dyn Node>>>,
    name: [u8; 4],
    sync_level: u8,
    // TODO: Implement mutex
}

impl Mutex {
    pub(crate) fn new(
        parent: Option<&Rc<RefCell<dyn Node>>>,
        name: [u8; 4],
        sync_level: u8,
    ) -> Rc<RefCell<dyn Node>> {
        Rc::new(RefCell::new(Mutex {
            parent: parent.map(|parent| Rc::downgrade(parent)),
            name,
            sync_level,
        }))
    }
}

impl Node for Mutex {
    fn name(&self) -> Option<[u8; 4]> {
        Some(self.name)
    }

    fn parent(&self) -> Option<Rc<RefCell<dyn Node>>> {
        self.parent.as_ref().map(|parent| parent.upgrade().unwrap())
    }
}

impl Display for Mutex {
    fn display(&self, f: &mut core::fmt::Formatter, depth: usize, last: bool) -> core::fmt::Result {
        display_prefix!(f, depth);
        write!(f, "Mutex (")?;
        display_name!(f, self.name);
        writeln!(f, ", {})", self.sync_level)
    }
}

impl_core_display!(Mutex);
