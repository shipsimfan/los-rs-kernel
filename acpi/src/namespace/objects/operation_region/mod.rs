use crate::namespace::{impl_core_display, Display};
use alloc::vec::Vec;

mod field;
mod region_space;

pub(crate) use field::{AccessType, Field, LockRule, UpdateRule};
pub(crate) use region_space::RegionSpace;

pub(crate) struct OperationRegion {
    name: Option<[u8; 4]>,
    offset: usize,
    length: usize,
    region_space: RegionSpace,
    fields: Vec<Field>,
}

impl OperationRegion {
    pub(crate) fn new(
        name: Option<[u8; 4]>,
        offset: usize,
        length: usize,
        region_space: RegionSpace,
    ) -> Self {
        OperationRegion {
            name,
            offset,
            length,
            region_space,
            fields: Vec::new(),
        }
    }

    pub(super) fn name(&self) -> Option<[u8; 4]> {
        self.name
    }

    pub(crate) fn add_field(&mut self, field: Field) {
        self.fields.push(field);
    }
}

impl Display for OperationRegion {
    fn display(&self, f: &mut core::fmt::Formatter, depth: usize) -> core::fmt::Result {
        self.display_prefix(f, depth)?;
        write!(f, "Operation Region")?;
        self.display_name(f, self.name)?;
        writeln!(
            f,
            " at {}:{:#X}-{:#X}",
            self.region_space,
            self.offset,
            self.offset + self.length
        )
    }
}

impl_core_display!(OperationRegion);
