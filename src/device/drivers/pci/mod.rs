use core::convert::{TryFrom, TryInto};

use alloc::{boxed::Box, sync::Arc};

use crate::{error, locks::Mutex, log, logln};

const ADDRESS_PORT: u16 = 0xCF8;
const DATA_PORT: u16 = 0xCFC;

#[repr(u8)]
pub enum Register {
    VendorID = 0x00,
    DeviceID = 0x02,
    Command = 0x04,
    Status = 0x06,
    Revision = 0x08,
    ProgIF = 0x09,
    SubClass = 0x0A,
    Class = 0x0B,
    CacheLineSize = 0x0C,
    LatencyTimer = 0x0D,
    HeaderType = 0x0E,
    BIST = 0x0F,
    BAR0 = 0x10,
    BAR1 = 0x14,
    BAR2 = 0x18,
    BAR3 = 0x1C,
    BAR4 = 0x20,
    BAR5 = 0x24,
    InterruptLine = 0x3C,
    InterruptPin = 0x3D,
    SecondaryBusNumber = 0x1A,
}

struct PCIDevice {
    bus: u8,
    device: u8,
    function: u8,
}

struct PCIBus;

fn read_config_b(bus: u8, device: u8, function: u8, offset: Register) -> u8 {
    let offset = offset as u8;
    let address = ((bus as u32) << 16)
        | ((device as u32) << 11)
        | ((function as u32) << 8)
        | ((offset as u32) & 0xFC)
        | 0x80000000;

    crate::device::outd(ADDRESS_PORT, address);
    let tmp = crate::device::ind(DATA_PORT);
    (tmp.wrapping_shr(((offset & 3) * 8) as u32) & 0xFF) as u8
}

fn read_config_w(bus: u8, device: u8, function: u8, offset: Register) -> u16 {
    let offset = offset as u8;
    let address = ((bus as u32) << 16)
        | ((device as u32) << 11)
        | ((function as u32) << 8)
        | ((offset as u32) & 0xFC)
        | 0x80000000;

    crate::device::outd(ADDRESS_PORT, address);
    let tmp = crate::device::ind(DATA_PORT);
    (tmp.wrapping_shr(((offset & 2) * 8) as u32) & 0xFFFF) as u16
}

fn read_config_d(bus: u8, device: u8, function: u8, offset: Register) -> u32 {
    let offset = offset as u8;
    let address = ((bus as u32) << 16)
        | ((device as u32) << 11)
        | ((function as u32) << 8)
        | ((offset as u32) & 0xFC)
        | 0x80000000;

    crate::device::outd(ADDRESS_PORT, address);
    crate::device::ind(DATA_PORT)
}

fn write_config_b(bus: u8, device: u8, function: u8, offset: Register, value: u8) {
    let offset = offset as u8;
    let address = ((bus as u32) << 16)
        | ((device as u32) << 11)
        | ((function as u32) << 8)
        | ((offset as u32) & 0xFC)
        | 0x80000000;

    let bit_shift = ((offset & 3) * 8) as u32;

    crate::device::outd(ADDRESS_PORT, address);
    let value =
        crate::device::ind(DATA_PORT) & !(0xFF << bit_shift) | ((value as u32) << bit_shift);

    crate::device::outd(ADDRESS_PORT, address);
    crate::device::outd(DATA_PORT, value);
}

fn write_config_w(bus: u8, device: u8, function: u8, offset: Register, value: u16) {
    let offset = offset as u8;
    let address = ((bus as u32) << 16)
        | ((device as u32) << 11)
        | ((function as u32) << 8)
        | ((offset as u32) & 0xFC)
        | 0x80000000;

    let bit_shift = ((offset & 2) * 8) as u32;

    crate::device::outd(ADDRESS_PORT, address);
    let value =
        crate::device::ind(DATA_PORT) & !(0xFF << bit_shift) | ((value as u32) << bit_shift);

    crate::device::outd(ADDRESS_PORT, address);
    crate::device::outd(DATA_PORT, value);
}

fn write_config_d(bus: u8, device: u8, function: u8, offset: Register, value: u32) {
    let offset = offset as u8;
    let address = ((bus as u32) << 16)
        | ((device as u32) << 11)
        | ((function as u32) << 8)
        | ((offset as u32) & 0xFC)
        | 0x80000000;

    crate::device::outd(ADDRESS_PORT, address);
    crate::device::outd(DATA_PORT, value);
}

fn check_pci_function(bus: u8, device: u8, function: u8) {
    let new_device: Arc<Mutex<Box<dyn crate::device::Device>>> =
        Arc::new(Mutex::new(Box::new(PCIDevice::new(bus, device, function))));

    let class = read_config_b(bus, device, function, Register::Class);
    let sub_class = read_config_b(bus, device, function, Register::SubClass);

    let path = alloc::format!("/pci/{:X}_{:X}", class, sub_class);
    match crate::device::register_device(&path, new_device) {
        Ok(()) => {}
        Err(error) => {
            logln!("Error while registering PCI device: {}", error);
            return;
        }
    }

    if class == 0x06 && sub_class == 0x04 {
        let secondary_bus = read_config_b(bus, device, function, Register::SecondaryBusNumber);
        check_pci_bus(secondary_bus);
    }
}

fn check_pci_device(bus: u8, device: u8) {
    let mut function = 0;

    let vendor_id = read_config_w(bus, device, function, Register::VendorID);
    if vendor_id == 0xFFFF {
        return;
    }

    check_pci_function(bus, device, function);
    let header_type = read_config_b(bus, device, function, Register::HeaderType);
    if (header_type & 0x80) != 0 {
        function = 1;
        while function < 8 {
            if read_config_w(bus, device, function, Register::VendorID) != 0xFFFF {
                check_pci_function(bus, device, function);
            }

            function += 1;
        }
    }
}

fn check_pci_bus(bus: u8) {
    let mut device = 0;
    while device < 32 {
        check_pci_device(bus, device);
        device += 1;
    }
}

pub fn initialize() {
    log!("Initializing PCI . . . ");

    match crate::device::register_device("/pci", Arc::new(Mutex::new(Box::new(PCIBus {})))) {
        Ok(()) => {}
        Err(error) => {
            logln!("Error while registering PCI bus: {}", error);
            return;
        }
    }

    let header_type = read_config_b(0, 0, 0, Register::HeaderType);
    if (header_type & 0x80) == 0 {
        check_pci_bus(0);
    } else {
        let mut function = 0;
        while function < 8 {
            if read_config_w(0, 0, function, Register::VendorID) != 0xFFFF {
                break;
            }

            check_pci_bus(function);

            function += 1;
        }
    }

    logln!("\x1B2A2]OK\x1B]!");
}

impl TryFrom<u8> for Register {
    type Error = error::Status;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x00 => Ok(Register::VendorID),
            0x02 => Ok(Register::DeviceID),
            0x04 => Ok(Register::Command),
            0x06 => Ok(Register::Status),
            0x08 => Ok(Register::Revision),
            0x09 => Ok(Register::ProgIF),
            0x0A => Ok(Register::SubClass),
            0x0B => Ok(Register::Class),
            0x0C => Ok(Register::CacheLineSize),
            0x0D => Ok(Register::LatencyTimer),
            0x0E => Ok(Register::HeaderType),
            0x0F => Ok(Register::BIST),
            0x10 => Ok(Register::BAR0),
            0x14 => Ok(Register::BAR1),
            0x18 => Ok(Register::BAR2),
            0x1C => Ok(Register::BAR3),
            0x20 => Ok(Register::BAR4),
            0x24 => Ok(Register::BAR5),
            0x3C => Ok(Register::InterruptLine),
            0x3D => Ok(Register::InterruptPin),
            0x1A => Ok(Register::SecondaryBusNumber),
            _ => Err(error::Status::OutOfRange),
        }
    }
}

impl PCIDevice {
    pub fn new(bus: u8, device: u8, function: u8) -> Self {
        PCIDevice {
            bus: bus,
            device: device,
            function: function,
        }
    }
}

impl crate::device::Device for PCIDevice {
    fn read(&self, _: usize, _: &mut [u8]) -> error::Result<()> {
        Err(error::Status::NotSupported)
    }

    fn write(&mut self, _: usize, _: &[u8]) -> error::Result<()> {
        Err(error::Status::NotSupported)
    }

    fn read_register(&mut self, address: usize) -> error::Result<usize> {
        let register = ((address & 0xFF) as u8).try_into()?;
        match register {
            Register::Class
            | Register::SubClass
            | Register::ProgIF
            | Register::Revision
            | Register::BIST
            | Register::HeaderType
            | Register::LatencyTimer
            | Register::CacheLineSize
            | Register::InterruptLine
            | Register::SecondaryBusNumber
            | Register::InterruptPin => {
                Ok(read_config_b(self.bus, self.device, self.function, register) as usize)
            }
            Register::Status | Register::Command | Register::DeviceID | Register::VendorID => {
                Ok(read_config_w(self.bus, self.device, self.function, register) as usize)
            }
            Register::BAR0
            | Register::BAR1
            | Register::BAR2
            | Register::BAR3
            | Register::BAR4
            | Register::BAR5 => {
                Ok(read_config_d(self.bus, self.device, self.function, register) as usize)
            }
        }
    }

    fn write_register(&mut self, address: usize, value: usize) -> error::Result<()> {
        let register = ((address & 0xFF) as u8).try_into()?;
        match register {
            Register::Class
            | Register::SubClass
            | Register::ProgIF
            | Register::Revision
            | Register::BIST
            | Register::HeaderType
            | Register::LatencyTimer
            | Register::CacheLineSize
            | Register::InterruptLine
            | Register::SecondaryBusNumber
            | Register::InterruptPin => Ok(write_config_b(
                self.bus,
                self.device,
                self.function,
                register,
                (value & 0xFF) as u8,
            )),
            Register::Status | Register::Command | Register::DeviceID | Register::VendorID => {
                Ok(write_config_w(
                    self.bus,
                    self.device,
                    self.function,
                    register,
                    (value & 0xFFFF) as u16,
                ))
            }
            Register::BAR0
            | Register::BAR1
            | Register::BAR2
            | Register::BAR3
            | Register::BAR4
            | Register::BAR5 => Ok(write_config_d(
                self.bus,
                self.device,
                self.function,
                register,
                (value & 0xFFFFFFFF) as u32,
            )),
        }
    }

    fn ioctrl(&mut self, _: usize, _: usize) -> error::Result<usize> {
        Err(error::Status::NotSupported)
    }
}

impl crate::device::Device for PCIBus {
    fn read(&self, _: usize, _: &mut [u8]) -> error::Result<()> {
        Err(error::Status::NotSupported)
    }

    fn write(&mut self, _: usize, _: &[u8]) -> error::Result<()> {
        Err(error::Status::NotSupported)
    }

    fn read_register(&mut self, _: usize) -> error::Result<usize> {
        Err(error::Status::NotSupported)
    }

    fn write_register(&mut self, _: usize, _: usize) -> error::Result<()> {
        Err(error::Status::NotSupported)
    }

    fn ioctrl(&mut self, _: usize, _: usize) -> error::Result<usize> {
        Err(error::Status::NotSupported)
    }
}
