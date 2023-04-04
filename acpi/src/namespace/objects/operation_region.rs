use crate::{
    interpreter::Integer,
    namespace::{display_name, display_prefix, impl_core_display, Display, Node},
    parser::{FieldFlags, RegionSpace},
};
use alloc::{
    rc::{Rc, Weak},
    vec::Vec,
};
use core::cell::RefCell;

pub(crate) struct OperationRegion {
    parent: Option<Weak<RefCell<dyn Node>>>,
    name: [u8; 4],
    space: RegionSpace,
    offset: Integer,
    length: Integer,
    fields: Vec<Field>,
}

pub(crate) struct Field {
    flags: FieldFlags,
    units: Vec<u8>,
}

impl OperationRegion {
    pub(crate) fn new(
        parent: Option<&Rc<RefCell<dyn Node>>>,
        name: [u8; 4],
        space: RegionSpace,
        offset: Integer,
        length: Integer,
    ) -> Rc<RefCell<dyn Node>> {
        Rc::new(RefCell::new(OperationRegion {
            parent: parent.map(|parent| Rc::downgrade(parent)),
            name,
            space,
            offset,
            length,
            fields: Vec::new(),
        }))
    }

    pub(crate) fn add_field(&mut self, field: Field) {
        self.fields.push(field);
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
    fn display(&self, f: &mut core::fmt::Formatter, depth: usize, last: bool) -> core::fmt::Result {
        display_prefix!(f, depth);
        write!(f, "Operation Region (")?;
        display_name!(f, self.name);
        write!(f, ", {}, {}, {}) {{", self.space, self.offset, self.length)?;

        if self.fields.len() == 0 {
            return writeln!(f, "}}");
        } else {
            writeln!(f)?;
        }

        for field in &self.fields {
            field.display(f, depth + 1, false)?;
        }

        display_prefix!(f, depth);
        writeln!(f, "}}")?;

        if !last {
            writeln!(f)
        } else {
            Ok(())
        }
    }
}

impl Field {
    pub(crate) fn new(flags: FieldFlags, units: Vec<u8>) -> Self {
        Field { flags, units }
    }
}

impl Display for Field {
    fn display(&self, f: &mut core::fmt::Formatter, depth: usize, _: bool) -> core::fmt::Result {
        display_prefix!(f, depth);
        writeln!(
            f,
            "Field ({}) {{ {} bytes... }}",
            self.flags,
            self.units.len()
        )
    }
}

impl_core_display!(OperationRegion);
impl_core_display!(Field);
