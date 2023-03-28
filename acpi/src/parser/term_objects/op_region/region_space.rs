#[derive(Clone, Copy)]
pub(crate) enum RegionSpace {
    SystemMemory,
    SystemIO,
    PCIConfig,
    EmbeddedControl,
    SMBus,
    SystemCMOS,
    PCIBarTarget,
    IPMI,
    GeneralPurposeIO,
    GenericSerialBus,
    PCC,
    Other(u8),
}

impl RegionSpace {
    pub(crate) fn from_u8(region_space: u8) -> Self {
        match region_space {
            0x00 => RegionSpace::SystemMemory,
            0x01 => RegionSpace::SystemIO,
            0x02 => RegionSpace::PCIConfig,
            0x03 => RegionSpace::EmbeddedControl,
            0x04 => RegionSpace::SMBus,
            0x05 => RegionSpace::SystemCMOS,
            0x06 => RegionSpace::PCIBarTarget,
            0x07 => RegionSpace::IPMI,
            0x08 => RegionSpace::GeneralPurposeIO,
            0x09 => RegionSpace::GenericSerialBus,
            0x0A => RegionSpace::PCC,
            _ => RegionSpace::Other(region_space),
        }
    }
}

impl core::fmt::Display for RegionSpace {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            RegionSpace::SystemMemory => write!(f, "System Memory"),
            RegionSpace::SystemIO => write!(f, "System IO"),
            RegionSpace::PCIConfig => write!(f, "PCI Config"),
            RegionSpace::EmbeddedControl => write!(f, "Embedded Control"),
            RegionSpace::SMBus => write!(f, "SM Bus"),
            RegionSpace::SystemCMOS => write!(f, "System CMOS"),
            RegionSpace::PCIBarTarget => write!(f, "PCI BAR Target"),
            RegionSpace::IPMI => write!(f, "IPMI"),
            RegionSpace::GeneralPurposeIO => write!(f, "General Purpose I/O"),
            RegionSpace::GenericSerialBus => write!(f, "Generic Serial Bus"),
            RegionSpace::PCC => write!(f, "PCC"),
            RegionSpace::Other(value) => write!(f, "Other ({:#X})", value),
        }
    }
}
