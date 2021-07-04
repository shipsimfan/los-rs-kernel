use super::{constants::*, controller};
use crate::{
    device::{self, Device, DeviceBox},
    error,
    locks::Mutex,
    logln, time,
};
use alloc::{boxed::Box, format, string::String, sync::Arc};

pub struct ATA {
    controller: DeviceBox,
    channel: Channel,
    drive: Drive,
    capabilities: u16,
}

#[derive(Debug, PartialEq)]
enum LBAMode {
    LBA48([usize; 6]),
    LBA24([usize; 6], u8),
    CHS([usize; 6], u8),
}

const SECTOR_SIZE: usize = 512;

impl ATA {
    pub fn create(
        channel: Channel,
        drive: Drive,
        capabilities: u16,
        size: usize,
        model: String,
    ) -> error::Result {
        let controller = device::get_device(super::IDE_PATH)?;

        let path = format!("/ide/{}", model.trim());
        let _size = size * SECTOR_SIZE;

        device::register_device(
            &path,
            Arc::new(Mutex::new(Box::new(ATA {
                controller: controller,
                channel: channel,
                drive: drive,
                capabilities: capabilities,
            }))),
        )?;

        Ok(())
    }
}

impl Device for ATA {
    fn read(&self, lba: usize, buffer: &mut [u8]) -> error::Result {
        let num_sectors = buffer.len() / SECTOR_SIZE;

        // Select lba_type
        let lba_mode = if lba >= 0x10000000 {
            LBAMode::LBA48([
                (lba.wrapping_shr(0) & 0xFF),
                (lba.wrapping_shr(8) & 0xFF),
                (lba.wrapping_shr(16) & 0xFF),
                (lba.wrapping_shr(24) & 0xFF),
                (lba.wrapping_shr(30) & 0xFF),
                (lba.wrapping_shr(40) & 0xFF),
            ])
        } else if self.capabilities & 0x200 != 0 {
            LBAMode::LBA24(
                [
                    (lba.wrapping_shr(0) & 0xFF),
                    (lba.wrapping_shr(8) & 0xFF),
                    (lba.wrapping_shr(16) & 0xFF),
                    0,
                    0,
                    0,
                ],
                (lba.wrapping_shr(24) & 0x0F) as u8,
            )
        } else {
            let sector = (lba % 63) + 1;
            let cylinder = (lba + 1 - sector) / (16 * 63);

            LBAMode::CHS(
                [
                    sector,
                    (cylinder.wrapping_shr(0) & 0xFF),
                    (cylinder.wrapping_shr(8) & 0xFF),
                    0,
                    0,
                    0,
                ],
                ((lba + 1 - sector) % (16 * 63) / 63) as u8,
            )
        };

        // Get the IDE controller
        let mut controller = self.controller.lock();

        // Disable IRQs
        controller.ioctrl(
            controller::IOCTRL_CLEAR_CHANNEL_INTERRUPT,
            self.channel.clone() as usize,
        )?;
        controller.write_register(self.channel.reg(REGISTER_CONTROL), 0x02)?;

        // Wait until drive is free
        while controller.read_register(self.channel.reg(REGISTER_STATUS))? & STATUS_BUSY != 0 {}

        // Select drive
        controller.write_register(
            self.channel.reg(REGISTER_DRIVE_SELECT),
            ((self.drive.clone() as usize) << 4)
                | (match &lba_mode {
                    LBAMode::CHS(_, head) => 0xA0 | head,
                    LBAMode::LBA24(_, head) => 0xE0 | head,
                    LBAMode::LBA48(_) => 0xE0,
                } as usize),
        )?;
        time::sleep(2);

        // Write parameters
        let lba_io = match &lba_mode {
            LBAMode::LBA48(io) => {
                controller.write_register(self.channel.reg(REGISTER_SECTOR_COUNT_1), 0)?;
                controller.write_register(self.channel.reg(REGISTER_LBA_3), io[3])?;
                controller.write_register(self.channel.reg(REGISTER_LBA_4), io[4])?;
                controller.write_register(self.channel.reg(REGISTER_LBA_5), io[5])?;
                io
            }
            LBAMode::LBA24(io, _) => io,
            LBAMode::CHS(io, _) => io,
        };

        controller.write_register(self.channel.reg(REGISTER_SECTOR_COUNT_0), 0)?;
        controller.write_register(self.channel.reg(REGISTER_LBA_0), lba_io[0])?;
        controller.write_register(self.channel.reg(REGISTER_LBA_1), lba_io[1])?;
        controller.write_register(self.channel.reg(REGISTER_LBA_2), lba_io[2])?;

        // Write the command
        let command = match lba_mode {
            LBAMode::CHS(_, _) | LBAMode::LBA24(_, _) => COMMAND_READ_PIO,
            LBAMode::LBA48(_) => COMMAND_READ_PIO_EXT,
        };
        controller.write_register(self.channel.reg(REGISTER_COMMAND), command)?;

        let mut index = 0;
        for _ in 0..num_sectors {
            if controller.ioctrl(
                controller::IOCTRL_ADVANCED_POLL,
                self.channel.clone() as usize,
            )? != 0
            {
                return Err(error::Status::DeviceError);
            }

            controller.read(
                self.channel.reg(REGISTER_DATA),
                &mut buffer[index..index + SECTOR_SIZE],
            )?;

            index += SECTOR_SIZE;
        }

        Ok(())
    }

    fn write(&mut self, _: usize, _: &[u8]) -> error::Result {
        Err(error::Status::NotSupported)
    }

    fn read_register(&mut self, _: usize) -> Result<usize, error::Status> {
        Err(error::Status::NotSupported)
    }

    fn write_register(&mut self, _: usize, _: usize) -> error::Result {
        Err(error::Status::NotSupported)
    }

    fn ioctrl(&mut self, _: usize, _: usize) -> Result<usize, error::Status> {
        Err(error::Status::NotSupported)
    }
}
