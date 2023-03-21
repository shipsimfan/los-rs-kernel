use crate::namespace::{impl_core_display, Display};

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

pub(crate) struct OperationRegion {
    name: Option<[u8; 4]>,
    offset: usize,
    length: usize,
    region_space: RegionSpace,
}

impl OperationRegion {
    pub(crate) fn new(
        name: Option<[u8; 4]>,
        offset: usize,
        length: usize,
        region_space: RegionSpace,
    ) -> Self {
        OperationRegion {
            name,
            offset,
            length,
            region_space,
        }
    }

    pub(super) fn name(&self) -> Option<[u8; 4]> {
        self.name
    }
}

impl Display for OperationRegion {
    fn display(&self, f: &mut core::fmt::Formatter, depth: usize) -> core::fmt::Result {
        self.display_prefix(f, depth)?;
        write!(f, "Operation Region")?;
        self.display_name(f, self.name)?;
        writeln!(
            f,
            " at {}:{:#X}-{:#X}",
            self.region_space,
            self.offset,
            self.offset + self.length
        )
    }
}

impl_core_display!(OperationRegion);

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
