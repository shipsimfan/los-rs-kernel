use super::constants::*;
use crate::{
    device::{self, Device, DeviceBox},
    error,
    locks::Mutex,
};
use alloc::{boxed::Box, format, string::String, sync::Arc};

pub struct ATAPI {
    _controller: DeviceBox,
    _channel: Channel,
    _drive: Drive,
    _capabilities: u16,
    size: usize,
}

const SECTOR_SIZE: usize = 2048;

impl ATAPI {
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
            Arc::new(Mutex::new(Box::new(ATAPI {
                _controller: controller,
                _channel: channel,
                _drive: drive,
                _capabilities: capabilities,
                size: size,
            }))),
        )
    }
}

impl Device for ATAPI {
    fn read(&self, _: usize, _: &mut [u8]) -> error::Result<()> {
        Err(error::Status::NotImplemented)
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
