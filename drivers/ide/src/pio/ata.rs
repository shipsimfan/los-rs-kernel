use crate::constants::*;
use alloc::{boxed::Box, format, string::String};
use base::{
    error::IDE_DRIVER_MODULE_NUMBER,
    multi_owner::{Owner, Reference},
};
use core::u8;
use device::Device;
use process::{Mutex, ProcessTypes};

#[derive(Debug)]
enum ATAError {
    IOError,
    NotSupported,
    InvalidIOCtrl,
    NoDevice,
}

pub struct ATA<T: ProcessTypes + 'static> {
    controller: Reference<Box<dyn Device>, Mutex<Box<dyn Device>, T>>,
    channel: Channel,
    drive: Drive,
    capabilities: u16,
    size: usize,
}

const SECTOR_SIZE: usize = 512;

impl<T: ProcessTypes + 'static> ATA<T> {
    pub fn create(
        channel: Channel,
        drive: Drive,
        capabilities: u16,
        size: usize,
        _model: String,
    ) -> base::error::Result<()> {
        let controller = device::get_device::<T>(IDE_PATH)?;

        let path = format!("/ide/{}_{}", channel, drive);
        let size = size * SECTOR_SIZE;

        device::register_device::<T>(
            &path,
            Owner::new(Box::new(ATA {
                controller: controller,
                channel: channel,
                drive: drive,
                capabilities: capabilities,
                size: size,
            }) as Box<dyn Device>),
        )
    }
}

impl<T: ProcessTypes + 'static> Device for ATA<T> {
    fn read(&self, lba: usize, buffer: &mut [u8]) -> base::error::Result<usize> {
        let channel = self.channel.clone();
        let slavebit = self.drive.clone() as usize;
        let num_sects = (buffer.len() + SECTOR_SIZE - 1) / SECTOR_SIZE;

        match self.controller.lock(|controller| {
            // Disable IRQ
            match controller.ioctrl(
                crate::controller::IOCTRL_CLEAR_CHANNEL_INTERRUPT,
                channel.clone() as usize,
            ) {
                Ok(_) => {}
                Err(error) => return Err(error),
            };

            let lba_mode;
            let lba_io;
            let head;
            let cyl;
            let sect;
            if lba >= 0x10000000 {
                lba_mode = 2;
                lba_io = [
                    ((lba & 0x0000000000FF) >> 00) as u8,
                    ((lba & 0x00000000FF00) >> 08) as u8,
                    ((lba & 0x000000FF0000) >> 16) as u8,
                    ((lba & 0x0000FF000000) >> 24) as u8,
                    ((lba & 0x00FF00000000) >> 32) as u8,
                    ((lba & 0xFF0000000000) >> 40) as u8,
                ];
                head = 0;
            } else if self.capabilities & 0x200 != 0 {
                lba_mode = 1;
                lba_io = [
                    ((lba & 0x00000FF) >> 00) as u8,
                    ((lba & 0x000FF00) >> 08) as u8,
                    ((lba & 0x0FF0000) >> 16) as u8,
                    0,
                    0,
                    0,
                ];
                head = (lba & 0xF000000) >> 24;
            } else {
                lba_mode = 0;
                sect = (lba % 63) + 1;
                cyl = (lba + 1 - sect) / (16 * 63);
                lba_io = [
                    sect as u8,
                    ((cyl & 0x00FF) >> 0) as u8,
                    ((cyl & 0xFF00) >> 8) as u8,
                    0,
                    0,
                    0,
                ];
                head = (lba + 1 - sect) % (16 * 63) / 63;
            }

            // Poll
            while controller.read_register(channel.reg(REGISTER_STATUS))? & STATUS_BUSY != 0 {}

            // Select drive
            if lba_mode == 0 {
                controller.write_register(
                    channel.reg(REGISTER_DRIVE_SELECT),
                    0xA0 | (slavebit << 4) | head,
                )?;
            } else {
                controller.write_register(
                    channel.reg(REGISTER_DRIVE_SELECT),
                    0xE0 | (slavebit << 4) | head,
                )?;
            }

            // Write parameters
            if lba_mode == 2 {
                controller.write_register(channel.reg(REGISTER_SECTOR_COUNT_1), 0)?;
                controller.write_register(channel.reg(REGISTER_LBA_3), lba_io[3] as usize)?;
                controller.write_register(channel.reg(REGISTER_LBA_4), lba_io[4] as usize)?;
                controller.write_register(channel.reg(REGISTER_LBA_5), lba_io[5] as usize)?;
            }

            controller.write_register(channel.reg(REGISTER_SECTOR_COUNT_0), num_sects)?;
            controller.write_register(channel.reg(REGISTER_LBA_0), lba_io[0] as usize)?;
            controller.write_register(channel.reg(REGISTER_LBA_1), lba_io[1] as usize)?;
            controller.write_register(channel.reg(REGISTER_LBA_2), lba_io[2] as usize)?;

            // Select and send command
            controller.write_register(
                channel.reg(REGISTER_COMMAND),
                match lba_mode {
                    0 => COMMAND_READ_PIO,
                    1 => COMMAND_READ_PIO,
                    _ => COMMAND_READ_PIO_EXT,
                },
            )?;

            for i in 0..num_sects {
                if controller.ioctrl(
                    crate::controller::IOCTRL_ADVANCED_POLL,
                    self.channel.clone() as usize,
                )? != 0
                {
                    return Err(Box::new(ATAError::IOError));
                }

                controller.read(
                    self.channel.reg(REGISTER_DATA),
                    &mut buffer[i * SECTOR_SIZE..(i + 1) * SECTOR_SIZE],
                )?;
            }

            Ok(buffer.len())
        }) {
            Some(result) => result,
            None => Err(Box::new(ATAError::NoDevice)),
        }
    }

    fn write(&mut self, lba: usize, buffer: &[u8]) -> base::error::Result<usize> {
        let channel = self.channel.clone();
        let slavebit = self.drive.clone() as usize;
        let num_sects = (buffer.len() + SECTOR_SIZE - 1) / SECTOR_SIZE;

        match self.controller.lock(|controller| {
            // Disable IRQ
            controller.ioctrl(
                crate::controller::IOCTRL_CLEAR_CHANNEL_INTERRUPT,
                channel.clone() as usize,
            )?;

            let lba_mode;
            let lba_io;
            let cyl;
            let head;
            let sect;

            if lba >= 0x10000000 {
                lba_mode = 2;
                lba_io = [
                    ((lba & 0x0000000000FF) >> 00) as u8,
                    ((lba & 0x00000000FF00) >> 08) as u8,
                    ((lba & 0x000000FF0000) >> 16) as u8,
                    ((lba & 0x0000FF000000) >> 24) as u8,
                    ((lba & 0x00FF00000000) >> 32) as u8,
                    ((lba & 0xFF0000000000) >> 40) as u8,
                ];
                head = 0;
            } else if self.capabilities & 0x200 != 0 {
                lba_mode = 1;
                lba_io = [
                    ((lba & 0x00000FF) >> 00) as u8,
                    ((lba & 0x000FF00) >> 08) as u8,
                    ((lba & 0x0FF0000) >> 16) as u8,
                    0,
                    0,
                    0,
                ];
                head = (lba & 0xF000000) >> 24;
            } else {
                lba_mode = 0;
                sect = (lba % 63) + 1;
                cyl = (lba + 1 - sect) / (16 * 63);
                lba_io = [
                    sect as u8,
                    ((cyl & 0x00FF) >> 0) as u8,
                    ((cyl & 0xFF00) >> 8) as u8,
                    0,
                    0,
                    0,
                ];
                head = (lba + 1 - sect) % (16 * 63) / 63;
            }

            // Poll
            while controller.read_register(channel.reg(REGISTER_STATUS))? & STATUS_BUSY != 0 {}

            // Select drive
            if lba_mode == 0 {
                controller.write_register(
                    channel.reg(REGISTER_DRIVE_SELECT),
                    0xA0 | (slavebit << 4) | head,
                )?;
            } else {
                controller.write_register(
                    channel.reg(REGISTER_DRIVE_SELECT),
                    0xE0 | (slavebit << 4) | head,
                )?;
            }

            // Write parameters
            if lba_mode == 2 {
                controller.write_register(channel.reg(REGISTER_SECTOR_COUNT_1), 0)?;
                controller.write_register(channel.reg(REGISTER_LBA_3), lba_io[3] as usize)?;
                controller.write_register(channel.reg(REGISTER_LBA_4), lba_io[4] as usize)?;
                controller.write_register(channel.reg(REGISTER_LBA_5), lba_io[5] as usize)?;
            }

            controller.write_register(channel.reg(REGISTER_SECTOR_COUNT_0), num_sects)?;
            controller.write_register(channel.reg(REGISTER_LBA_0), lba_io[0] as usize)?;
            controller.write_register(channel.reg(REGISTER_LBA_1), lba_io[1] as usize)?;
            controller.write_register(channel.reg(REGISTER_LBA_2), lba_io[2] as usize)?;

            // Select and send command
            controller.write_register(
                channel.reg(REGISTER_COMMAND),
                match lba_mode {
                    0 | 1 => COMMAND_WRITE_PIO,
                    _ => COMMAND_WRITE_PIO_EXT,
                },
            )?;

            for i in 0..num_sects {
                controller.ioctrl(
                    crate::controller::IOCTRL_POLL,
                    self.channel.clone() as usize,
                )?;
                controller.write(
                    self.channel.reg(REGISTER_DATA),
                    &buffer[i * SECTOR_SIZE..(i + 1) * SECTOR_SIZE],
                )?;
            }

            controller.write_register(
                REGISTER_COMMAND,
                match lba_mode {
                    0 | 1 => COMMAND_CACHE_FLUSH,
                    _ => COMMAND_CACHE_FLUSH_EXT,
                },
            )?;
            controller.ioctrl(
                crate::controller::IOCTRL_POLL,
                self.channel.clone() as usize,
            )?;

            Ok(buffer.len())
        }) {
            Some(result) => result,
            None => Err(Box::new(ATAError::NoDevice)),
        }
    }

    fn read_register(&mut self, _: usize) -> base::error::Result<usize> {
        Err(Box::new(ATAError::NotSupported))
    }

    fn write_register(&mut self, _: usize, _: usize) -> base::error::Result<()> {
        Err(Box::new(ATAError::NotSupported))
    }

    fn ioctrl(&mut self, code: usize, _: usize) -> base::error::Result<usize> {
        match code {
            0 => Ok(self.size),
            _ => Err(Box::new(ATAError::InvalidIOCtrl)),
        }
    }
}

impl base::error::Error for ATAError {
    fn module_number(&self) -> i32 {
        IDE_DRIVER_MODULE_NUMBER
    }

    fn error_number(&self) -> base::error::Status {
        match self {
            ATAError::IOError => base::error::Status::IOError,
            ATAError::InvalidIOCtrl => base::error::Status::InvalidIOCtrl,
            ATAError::NotSupported => base::error::Status::NotSupported,
            ATAError::NoDevice => base::error::Status::NoDevice,
        }
    }
}

impl core::fmt::Display for ATAError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            ATAError::IOError => write!(f, "I/O error"),
            ATAError::InvalidIOCtrl => write!(f, "Invalid I/O control for ATA"),
            ATAError::NotSupported => write!(f, "Not supported for ATA"),
            ATAError::NoDevice => write!(f, "No device"),
        }
    }
}
