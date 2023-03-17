use super::{IDENTITY_MAP_BOTTOM, IDENTITY_MAP_TOP, KERNEL_VMA};

#[derive(Clone, Copy)]
pub struct PhysicalAddress {
    address: usize,
}

impl PhysicalAddress {
    pub unsafe fn from_raw(address: usize) -> Self {
        PhysicalAddress { address }
    }

    pub fn new<T>(r#virtual: *const T) -> Self {
        let address = r#virtual as usize;

        if address < IDENTITY_MAP_BOTTOM || address > IDENTITY_MAP_TOP {
            panic!("Virtual address outside the identity map cannot currently be mapped to physical addresses");
        }

        PhysicalAddress {
            address: address - KERNEL_VMA,
        }
    }

    pub unsafe fn add(&mut self, bytes: usize) {
        self.address += bytes;
    }

    pub(in crate::memory) unsafe fn into_usize(self) -> usize {
        self.address
    }
}
