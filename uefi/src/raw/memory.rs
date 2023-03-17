use base::PhysicalAddress;

#[repr(u32)]
#[derive(Debug, Clone, Copy)]
pub enum MemoryClass {
    Reserved = 0,
    LoaderCode = 1,
    LoaderData = 2,
    BootSerivesCode = 3,
    BootServicesData = 4,
    RuntimeServicesCode = 5,
    RuntimeServiesData = 6,
    Conventional = 7,
    Unusable = 8,
    ACPIReclaim = 9,
    ACPINvs = 10,
    MMIO = 11,
    MMIOPort = 12,
    PALCode = 13,
    Persistent = 14,
    Max = 15,
}

#[repr(C)]
pub struct MemoryMap {
    size: usize,
    key: usize,
    descriptor_size: usize,
    descriptor_version: u32,
    address: PhysicalAddress,
}

#[repr(C)]
pub struct MemoryDescriptor {
    class: MemoryClass,
    physical_address: u64,
    virtual_address: u64,
    num_pages: u64,
    attribute: u64,
}

pub(crate) const EFI_MEMORY_SP: u64 = 0x0000000000040000;

impl MemoryMap {
    pub(crate) fn address(&self) -> PhysicalAddress {
        self.address
    }

    pub(crate) fn descriptor_size(&self) -> usize {
        self.descriptor_size
    }

    pub(crate) fn size(&self) -> usize {
        self.size
    }
}

impl MemoryDescriptor {
    pub(crate) fn physical_address(&self) -> u64 {
        self.physical_address
    }

    pub(crate) fn num_pages(&self) -> u64 {
        self.num_pages
    }

    pub(crate) fn class(&self) -> MemoryClass {
        self.class
    }

    pub(crate) fn attribute(&self) -> u64 {
        self.attribute
    }
}
