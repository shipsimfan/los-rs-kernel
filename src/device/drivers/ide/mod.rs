use alloc::{boxed::Box, sync::Arc};

use crate::{
    device::{self, Device, DeviceBox},
    error,
    locks::Mutex,
    log, logln, time,
};

enum IDEType {
    ATA,
    ATAPI,
}

struct ATADevice {}

struct ATAPIDevice {}

struct Channel {
    pub io: u16,
    pub control: u16,
    pub bus_master: u16,
    pub n_ien: u8,
}

struct IDEController {
    channels: [Channel; 2],
}

const ATA_REG_DATA: usize = 0x00;
const ATA_REG_ERROR: usize = 0x01;
const ATA_REG_FEATURES: usize = 0x01;
const ATA_REG_SECCOUNT0: usize = 0x02;
const ATA_REG_LBA0: usize = 0x03;
const ATA_REG_LBA1: usize = 0x04;
const ATA_REG_LBA2: usize = 0x05;
const ATA_REG_HDDEVSEL: usize = 0x06;
const ATA_REG_COMMAND: usize = 0x07;
const ATA_REG_STATUS: usize = 0x07;
const ATA_REG_SECCOUNT1: usize = 0x08;
const ATA_REG_LBA3: usize = 0x09;
const ATA_REG_LBA4: usize = 0x0A;
const ATA_REG_LBA5: usize = 0x0B;
const ATA_REG_CONTROL: usize = 0x0C;
const ATA_REG_ALTSTATUS: usize = 0x0C;
const ATA_REG_DEVADDRESS: usize = 0x0D;

const ATA_CMD_READ_PIO: usize = 0x20;
const ATA_CMD_READ_PIO_EXT: usize = 0x24;
const ATA_CMD_READ_DMA: usize = 0xC8;
const ATA_CMD_READ_DMA_EXT: usize = 0x25;
const ATA_CMD_WRITE_PIO: usize = 0x30;
const ATA_CMD_WRITE_PIO_EXT: usize = 0x34;
const ATA_CMD_WRITE_DMA: usize = 0xCA;
const ATA_CMD_WRITE_DMA_EXT: usize = 0x35;
const ATA_CMD_CACHE_FLUSH: usize = 0xE7;
const ATA_CMD_CACHE_FLUSH_EXT: usize = 0xEA;
const ATA_CMD_PACKET: usize = 0xA0;
const ATA_CMD_IDENTIFY_PACKET: usize = 0xA1;
const ATA_CMD_IDENTIFY: usize = 0xEC;

const ATA_SR_BSY: usize = 0x80;
const ATA_SR_DRDY: usize = 0x40;
const ATA_SR_DF: usize = 0x20;
const ATA_SR_DSC: usize = 0x10;
const ATA_SR_DRQ: usize = 0x08;
const ATA_SR_CORR: usize = 0x04;
const ATA_SR_IDX: usize = 0x02;
const ATA_SR_ERR: usize = 0x01;

pub fn initialize() {
    log!("Initializing IDE . . . ");

    let pci_ide_controller = match device::get_device("/pci/1_1") {
        Ok(device) => device,
        Err(error) => {
            logln!("Failed to get ide controller: {}", error);
            return;
        }
    };

    match IDEController::new(pci_ide_controller) {
        Ok(()) => logln!("\x1B2A2]OK\x1B]!"),
        Err(error) => logln!("Failed to create ide controller: {}", error),
    }
}

#[allow(unused_must_use)]
fn irq_handler(context: usize) {
    let channel_lock = context as *const Mutex<Box<dyn Device>>;
    let mut channel = unsafe { (*channel_lock).lock() };
    channel.ioctrl(1, 0);
}

fn ata_reg(register: usize, primary: bool) -> usize {
    if primary {
        register
    } else {
        0x100 | register
    }
}

impl IDEController {
    pub fn new(pci_device: DeviceBox) -> error::Result {
        // Get port numbers
        let mut device = pci_device.lock();

        let primary_io = device.read_register(super::pci::Register::BAR0 as usize)? as u16;
        let primary_control = device.read_register(super::pci::Register::BAR1 as usize)? as u16;

        let secondary_io = device.read_register(super::pci::Register::BAR2 as usize)? as u16;
        let secondary_control = device.read_register(super::pci::Register::BAR3 as usize)? as u16;

        let bus_master = device.read_register(super::pci::Register::BAR4 as usize)? as u16;

        drop(device);
        drop(pci_device);
        device::remove_device("/pci/1_1");

        // Create two channels
        let channels = [
            Channel::new(primary_io, primary_control, bus_master, true),
            Channel::new(secondary_io, secondary_control, bus_master + 8, false),
        ];

        // Create and register the ide controller
        let controller_lock: DeviceBox =
            Arc::new(Mutex::new(Box::new(IDEController { channels: channels })));
        device::register_device("/ide", controller_lock.clone())?;

        // Install interrupts
        crate::interrupts::irq::install_irq_handler(
            14,
            irq_handler,
            Arc::as_ptr(&controller_lock) as usize,
        );
        crate::interrupts::irq::install_irq_handler(
            15,
            irq_handler,
            Arc::as_ptr(&controller_lock) as usize,
        );

        // Disable interrupts
        let mut ide = controller_lock.lock();
        ide.write_register(ata_reg(ATA_REG_CONTROL, true), 2)?;
        ide.write_register(ata_reg(ATA_REG_CONTROL, false), 2)?;

        // Enumerate devices
        ide.ioctrl(0, 0)?; // Primary
        ide.ioctrl(0, 1)?; // Secondary

        Ok(())
    }

    pub fn enumerate_drives(&mut self, primary: bool) -> error::Result {
        // Check drives
        let mut i = 0;
        while i < 2 {
            // Select drive
            self.write_register(ata_reg(ATA_REG_HDDEVSEL, primary), 0xA0 | (i << 4))?;
            time::sleep(2);

            // Check device existence
            self.write_register(ata_reg(ATA_REG_COMMAND, primary), ATA_CMD_IDENTIFY)?;
            time::sleep(2);

            if self.read_register(ata_reg(ATA_REG_STATUS, primary))? == 0 {
                i += 1;
                continue;
            }

            let err = loop {
                let value = self.read_register(ata_reg(ATA_REG_STATUS, primary))?;

                if value & ATA_SR_ERR != 0 {
                    break 1;
                }

                if value & ATA_SR_BSY == 0 && value & ATA_SR_DRQ != 0 {
                    break 0;
                }
            };

            // Probe for ATAPI
            let drive_type = if err != 0 {
                let cl = self.read_register(ata_reg(ATA_REG_LBA1, primary))? as u8;
                let ch = self.read_register(ata_reg(ATA_REG_LBA2, primary))? as u8;

                if (cl == 0x14 && ch == 0xEB) || (cl == 0x69 && ch == 0x96) {
                    self.write_register(
                        ata_reg(ATA_REG_COMMAND, primary),
                        ATA_CMD_IDENTIFY_PACKET,
                    )?;
                    time::sleep(2);
                    IDEType::ATAPI
                } else {
                    i += 1;
                    continue;
                }
            } else {
                IDEType::ATA
            };

            match drive_type {
                IDEType::ATA => ATADevice::new()?,
                IDEType::ATAPI => ATAPIDevice::new()?,
            }

            i += 1;
        }

        Ok(())
    }
}

impl Device for IDEController {
    fn read(&self, _: usize, _: &mut [u8]) -> error::Result {
        Err(error::Status::NotSupported)
    }

    fn write(&mut self, _: usize, _: &[u8]) -> error::Result {
        Err(error::Status::NotSupported)
    }

    fn read_register(&mut self, address: usize) -> Result<usize, error::Status> {
        let reg = (address & 0xFF) as u16;
        let channel = address.wrapping_shr(8) & 1;

        if reg > 0x07 && reg < 0x0C {
            self.write_register(
                (address & 0x100) | ATA_REG_CONTROL,
                0x80 | self.channels[channel].n_ien as usize,
            )?;
        }

        let ret = if reg < 0x08 {
            device::inb(self.channels[channel].io + reg)
        } else if reg < 0x0C {
            device::inb(self.channels[channel].io + reg - 0x06)
        } else if reg < 0x0E {
            device::inb(self.channels[channel].control + reg - 0x0A)
        } else if reg < 0x16 {
            device::inb(self.channels[channel].bus_master + reg - 0x0E)
        } else {
            return Err(error::Status::InvalidArgument);
        };

        if reg > 0x07 && reg < 0x0C {
            self.write_register(
                (address & 0x100) | ATA_REG_CONTROL,
                self.channels[channel].n_ien as usize,
            )?;
        }

        Ok(ret as usize)
    }

    fn write_register(&mut self, address: usize, value: usize) -> error::Result {
        let reg = (address & 0xFF) as u16;
        let channel = address.wrapping_shr(8) & 1;
        let value = (value & 0xFF) as u8;

        if reg > 0x07 && reg < 0x0C {
            self.write_register(
                (address & 0x100) | ATA_REG_CONTROL,
                0x80 | self.channels[channel].n_ien as usize,
            )?;
        }

        if reg < 0x08 {
            device::outb(self.channels[channel].io + reg, value);
        } else if reg < 0x0C {
            device::outb(self.channels[channel].io + reg - 0x06, value);
        } else if reg < 0x0E {
            device::outb(self.channels[channel].control + reg - 0x0A, value);
        } else if reg < 0x16 {
            device::outb(self.channels[channel].bus_master + reg - 0x0E, value);
        } else {
            return Err(error::Status::InvalidArgument);
        };

        if reg > 0x07 && reg < 0x0C {
            self.write_register(
                (address & 0x100) | ATA_REG_CONTROL,
                self.channels[channel].n_ien as usize,
            )?;
        }

        Ok(())
    }

    fn ioctrl(&mut self, code: usize, argument: usize) -> error::Result {
        match code {
            0 => {
                if argument >= 2 {
                    Err(error::Status::InvalidArgument)
                } else {
                    self.enumerate_drives(argument == 0)
                }
            }
            _ => Err(error::Status::InvalidArgument),
        }
    }
}

impl Channel {
    pub fn new(io: u16, control: u16, bus_master: u16, primary: bool) -> Self {
        // Correct address
        let io = if io <= 1 {
            if primary {
                0x1F0
            } else {
                0x170
            }
        } else {
            io
        };

        let control = if control <= 1 {
            if primary {
                0x3F4
            } else {
                0x374
            }
        } else {
            control
        };

        // Create the channel
        Channel {
            io: io,
            control: control,
            bus_master: bus_master,
            n_ien: 0,
        }
    }
}

impl ATADevice {
    pub fn new() -> error::Result {
        logln!("Creating ATA device");

        Ok(())
    }
}

impl ATAPIDevice {
    pub fn new() -> error::Result {
        logln!("Creating ATAPI device");

        Ok(())
    }
}
