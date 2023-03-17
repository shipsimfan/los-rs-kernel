use crate::raw;
use base::PhysicalAddress;

pub struct MemoryDescriptor {
    address: PhysicalAddress,
    num_pages: usize,
    is_usable: bool,
}

impl From<*const raw::MemoryDescriptor> for MemoryDescriptor {
    fn from(raw: *const raw::MemoryDescriptor) -> Self {
        use raw::MemoryClass::*;

        unsafe {
            MemoryDescriptor {
                address: PhysicalAddress::from_raw((*raw).physical_address() as usize),
                num_pages: (*raw).num_pages() as usize,
                is_usable: match (*raw).class() {
                    LoaderCode | LoaderData | BootSerivesCode | BootServicesData
                    | RuntimeServicesCode | RuntimeServiesData | Conventional | Persistent => {
                        ((*raw).attribute() & raw::EFI_MEMORY_SP) == 0
                    }
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

    fn is_usable(&self) -> bool {
        self.is_usable
    }
}
