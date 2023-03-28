use crate::{
    namespace::{display_name, display_prefix, impl_core_display, Display, Node},
    parser::RegionSpace,
};
use alloc::rc::{Rc, Weak};
use core::cell::RefCell;

pub(crate) struct OperationRegion {
    parent: Option<Weak<RefCell<dyn Node>>>,
    name: [u8; 4],
    space: RegionSpace,
    offset: u64,
    length: u64,
}

impl OperationRegion {
    pub(crate) fn new(
        parent: Option<&Rc<RefCell<dyn Node>>>,
        name: [u8; 4],
        space: RegionSpace,
        offset: u64,
        length: u64,
    ) -> Rc<RefCell<dyn Node>> {
        Rc::new(RefCell::new(OperationRegion {
            parent: parent.map(|parent| Rc::downgrade(parent)),
            name,
            space,
            offset,
            length,
        }))
    }
}

impl Node for OperationRegion {
    fn parent(&self) -> Option<alloc::rc::Rc<RefCell<dyn Node>>> {
        self.parent.as_ref().map(|parent| parent.upgrade().unwrap())
    }

    fn name(&self) -> Option<[u8; 4]> {
        Some(self.name)
    }
}

impl Display for OperationRegion {
    fn display(&self, f: &mut core::fmt::Formatter, depth: usize, _: bool) -> core::fmt::Result {
        display_prefix!(f, depth);
        write!(f, "Operation Region (")?;
        display_name!(f, self.name);
        writeln!(f, ", {}, {}, {})", self.space, self.offset, self.length)
    }
}

impl_core_display!(OperationRegion);
