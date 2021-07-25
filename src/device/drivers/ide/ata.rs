use core::u8;

use super::{constants::*, controller};
use crate::{
    device::{self, Device, DeviceBox},
    error,
    locks::Mutex,
};
use alloc::{boxed::Box, format, string::String, sync::Arc};

pub struct ATA {
    controller: DeviceBox,
    channel: Channel,
    drive: Drive,
    capabilities: u16,
    size: usize,
}

const SECTOR_SIZE: usize = 512;

impl ATA {
    pub fn create(
        channel: Channel,
        drive: Drive,
        capabilities: u16,
        size: usize,
        _model: String,
    ) -> error::Result<()> {
        let controller = device::get_device(super::IDE_PATH)?;

        let path = format!("/ide/{}_{}", channel, drive);
        let size = size * SECTOR_SIZE;

        device::register_device(
            &path,
            Arc::new(Mutex::new(Box::new(ATA {
                controller: controller,
                channel: channel,
                drive: drive,
                capabilities: capabilities,
                size: size,
            }))),
        )
    }
}

impl Device for ATA {
    fn read(&self, lba: usize, buffer: &mut [u8]) -> error::Result<()> {
        let lba_mode;
        let lba_io;
        let channel = self.channel.clone();
        let slavebit = self.drive.clone() as usize;
        let mut controller = self.controller.lock();
        let cyl;
        let num_sects = (buffer.len() + SECTOR_SIZE - 1) / SECTOR_SIZE;
        let head;
        let sect;

        // Disable IRQ
        controller.ioctrl(
            controller::IOCTRL_CLEAR_CHANNEL_INTERRUPT,
            channel.clone() as usize,
        )?;

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
                controller::IOCTRL_ADVANCED_POLL,
                self.channel.clone() as usize,
            )? != 0
            {
                return Err(error::Status::IOError);
            }

            controller.read(
                self.channel.reg(REGISTER_DATA),
                &mut buffer[i * SECTOR_SIZE..(i + 1) * SECTOR_SIZE],
            )?;
        }

        Ok(())
    }

    fn write(&mut self, _: usize, _: &[u8]) -> error::Result<()> {
        Err(error::Status::NotImplemented)
    }

    fn read_register(&mut self, _: usize) -> error::Result<usize> {
        Err(error::Status::NotSupported)
    }

    fn write_register(&mut self, _: usize, _: usize) -> error::Result<()> {
        Err(error::Status::NotSupported)
    }

    fn ioctrl(&mut self, code: usize, _: usize) -> error::Result<usize> {
        match code {
            0 => Ok(self.size),
            _ => Err(error::Status::InvalidIOCtrl),
        }
    }
}
