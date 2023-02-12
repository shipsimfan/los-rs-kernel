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
    pub size: usize,
    pub key: usize,
    pub desc_size: usize,
    pub desc_version: u32,
    pub address: *const MemoryDescriptor,
}

#[repr(C)]
pub struct MemoryDescriptor {
    pub class: MemoryClass,
    pub physical_address: usize,
    pub virtual_address: usize,
    pub num_pages: usize,
    pub attribute: usize,
}
