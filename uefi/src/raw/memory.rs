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
    address: *const MemoryDescriptor,
}

#[repr(C)]
pub struct MemoryDescriptor {
    class: MemoryClass,
    physical_address: usize,
    virtual_address: usize,
    num_pages: usize,
    attribute: usize,
}

impl MemoryMap {
    pub(crate) fn address(&self) -> *const MemoryDescriptor {
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
    pub(crate) fn physical_address(&self) -> usize {
        self.physical_address
    }

    pub(crate) fn num_pages(&self) -> usize {
        self.num_pages
    }

    pub(crate) fn class(&self) -> MemoryClass {
        self.class
    }
}
