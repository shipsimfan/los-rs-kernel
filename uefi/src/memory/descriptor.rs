use crate::raw;
use base::PhysicalAddress;

pub struct MemoryDescriptor {
    address: PhysicalAddress,
    num_pages: usize,
    is_freeable: bool,
}

impl From<*const raw::MemoryDescriptor> for MemoryDescriptor {
    fn from(raw: *const raw::MemoryDescriptor) -> Self {
        use raw::MemoryClass::*;

        unsafe {
            MemoryDescriptor {
                address: (*raw).physical_address().into(),
                num_pages: (*raw).num_pages(),
                is_freeable: match (*raw).class() {
                    LoaderCode | LoaderData | BootSerivesCode | BootServicesData
                    | RuntimeServicesCode | RuntimeServiesData | Conventional | Persistent => true,
                    _ => false,
                },
            }
        }
    }
}

impl base::MemoryDescriptor for MemoryDescriptor {
    fn address(&self) -> PhysicalAddress {
        self.address
    }

    fn num_pages(&self) -> usize {
        self.num_pages
    }

    fn is_freeable(&self) -> bool {
        self.is_freeable
    }
}
