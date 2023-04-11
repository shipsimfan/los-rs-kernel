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
        }
    }
}
