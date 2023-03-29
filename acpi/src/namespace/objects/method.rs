use crate::namespace::{display_name, display_prefix, impl_core_display, Display, Node};
use alloc::rc::{Rc, Weak};
use core::cell::RefCell;

pub(crate) struct Method {
    parent: Option<Weak<RefCell<dyn Node>>>,
    name: [u8; 4],
    arg_count: u8,
    serialized: bool,
    sync_level: u8,
    method_size: usize,
}

impl Method {
    pub(crate) fn new(
        parent: Option<&Rc<RefCell<dyn Node>>>,
        name: [u8; 4],
        arg_count: u8,
        serialized: bool,
        sync_level: u8,
        method_size: usize,
    ) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Method {
            parent: parent.map(|parent| Rc::downgrade(parent)),
            name,
            arg_count,
            serialized,
            sync_level,
            method_size,
        }))
    }
}

impl Node for Method {
    fn name(&self) -> Option<[u8; 4]> {
        Some(self.name)
    }

    fn parent(&self) -> Option<alloc::rc::Rc<RefCell<dyn Node>>> {
        self.parent.as_ref().map(|parent| parent.upgrade().unwrap())
    }
}

impl Display for Method {
    fn display(&self, f: &mut core::fmt::Formatter, depth: usize, _: bool) -> core::fmt::Result {
        display_prefix!(f, depth);
        write!(f, "Method (")?;
        display_name!(f, self.name);
        writeln!(
            f,
            ", {}, {}, {}) {{ {} bytes... }}",
            self.arg_count, self.serialized, self.sync_level, self.method_size
        )
    }
}

impl_core_display!(Method);
