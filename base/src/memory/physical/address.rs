use core::ops::Deref;

pub(crate) struct PhysicalAddress(usize);

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
