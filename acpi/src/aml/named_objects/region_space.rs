use crate::aml::{next, ByteStream, Result};

pub(super) enum RegionSpace {
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
    OEMDefined(u8),
}

const SYSTEM_MEMORY: u8 = 0x00;
const SYSTEM_IO: u8 = 0x01;
const PCI_CONFIG: u8 = 0x02;
const EMBEDDED_CONTROL: u8 = 0x03;
const SM_BUS: u8 = 0x04;
const SYSTEM_CMOS: u8 = 0x05;
const PCI_BAR_TARGET: u8 = 0x06;
const IPMI: u8 = 0x07;
const GENERAL_PURPOSE_IO: u8 = 0x08;
const GENERIC_SERIAL_BUS: u8 = 0x09;
const PCC: u8 = 0x0A;

const OEM_MINIMUM: u8 = 0x80;

impl RegionSpace {
    pub(super) fn parse(stream: &mut ByteStream) -> Result<Self> {
        let c = next!(stream);
        if c >= OEM_MINIMUM {
            return Ok(RegionSpace::OEMDefined(c));
        }

        match c {
            SYSTEM_MEMORY => Ok(RegionSpace::SystemMemory),
            SYSTEM_IO => Ok(RegionSpace::SystemIO),
            PCI_CONFIG => Ok(RegionSpace::PCIConfig),
            EMBEDDED_CONTROL => Ok(RegionSpace::EmbeddedControl),
            SM_BUS => Ok(RegionSpace::SMBus),
            SYSTEM_CMOS => Ok(RegionSpace::SystemCMOS),
            PCI_BAR_TARGET => Ok(RegionSpace::PCIBarTarget),
            IPMI => Ok(RegionSpace::IPMI),
            GENERAL_PURPOSE_IO => Ok(RegionSpace::GeneralPurposeIO),
            GENERIC_SERIAL_BUS => Ok(RegionSpace::GenericSerialBus),
            PCC => Ok(RegionSpace::PCC),
            _ => Err(crate::aml::Error::UnexpectedByte(c)),
        }
    }
}

impl core::fmt::Display for RegionSpace {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            RegionSpace::SystemMemory => write!(f, "System Memory"),
            RegionSpace::SystemIO => write!(f, "System Memory"),
            RegionSpace::PCIConfig => write!(f, "System Memory"),
            RegionSpace::EmbeddedControl => write!(f, "System Memory"),
            RegionSpace::SMBus => write!(f, "System Memory"),
            RegionSpace::SystemCMOS => write!(f, "System Memory"),
            RegionSpace::PCIBarTarget => write!(f, "System Memory"),
            RegionSpace::IPMI => write!(f, "System Memory"),
            RegionSpace::GeneralPurposeIO => write!(f, "System Memory"),
            RegionSpace::GenericSerialBus => write!(f, "System Memory"),
            RegionSpace::PCC => write!(f, "System Memory"),
            RegionSpace::OEMDefined(value) => write!(f, "OEM {:#X}", value),
        }
    }
}
