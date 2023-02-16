use crate::memory::{PAGE_MASK, PAGE_SIZE};
use core::ops::Deref;

#[derive(Clone, Copy)]
pub struct PhysicalAddress(usize);

impl PhysicalAddress {
    pub(super) const fn null() -> Self {
        PhysicalAddress(0)
    }

    pub(super) fn from_page(page: usize) -> Self {
        PhysicalAddress(page * PAGE_SIZE)
    }

    pub(super) fn to_page(self) -> usize {
        (self.0 & PAGE_MASK) / PAGE_SIZE
    }
}

impl Deref for PhysicalAddress {
    type Target = usize;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<usize> for PhysicalAddress {
    fn from(value: usize) -> Self {
        PhysicalAddress(value)
    }
}

impl Into<usize> for PhysicalAddress {
    fn into(self) -> usize {
        self.0
    }
}

impl core::fmt::Debug for PhysicalAddress {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Display::fmt(self, f)
    }
}

impl core::fmt::Display for PhysicalAddress {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{:#X}", self.0)
    }
}
