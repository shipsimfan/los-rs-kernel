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

pub(crate) struct OperationRegion<'a> {
    parent: Option<Weak<RefCell<Node<'a>>>>,
    name: [u8; 4],
    space: RegionSpace,
    offset: Integer,
    length: Integer,
    fields: Vec<Field<'a>>,
}

pub(crate) struct Field<'a> {
    flags: FieldFlags,
    units: &'a [u8],
}

impl<'a> OperationRegion<'a> {
    pub(crate) fn new(
        parent: Option<&Rc<RefCell<Node<'a>>>>,
        name: [u8; 4],
        space: RegionSpace,
        offset: Integer,
        length: Integer,
    ) -> Rc<RefCell<Node<'a>>> {
        Rc::new(RefCell::new(Node::OperationRegion(OperationRegion {
            parent: parent.map(|parent| Rc::downgrade(parent)),
            name,
            space,
            offset,
            length,
            fields: Vec::new(),
        })))
    }

    pub(crate) fn add_field(&mut self, field: Field<'a>) {
        self.fields.push(field);
    }

    pub(in crate::namespace) fn parent(&self) -> Option<alloc::rc::Rc<RefCell<Node<'a>>>> {
        self.parent.as_ref().map(|parent| parent.upgrade().unwrap())
    }

    pub(in crate::namespace) fn name(&self) -> [u8; 4] {
        self.name
    }
}

impl<'a> Display for OperationRegion<'a> {
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

impl<'a> Field<'a> {
    pub(crate) fn new(flags: FieldFlags, units: &'a [u8]) -> Self {
        Field { flags, units }
    }
}

impl<'a> Display for Field<'a> {
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
