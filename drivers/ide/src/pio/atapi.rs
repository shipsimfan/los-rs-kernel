use crate::constants::*;
use alloc::{boxed::Box, format, string::String};
use base::{error::IDE_DRIVER_MODULE_NUMBER, multi_owner::Reference};
use device::Device;
use process::{Mutex, ProcessTypes};

#[derive(Debug)]
enum ATAPIError {
    NotImplemented,
    NotSupported,
    InvalidIOCtrl,
}

pub struct ATAPI<T: ProcessTypes + 'static> {
    _controller: Reference<Box<dyn Device>, Mutex<Box<dyn Device>, T>>,
    _channel: Channel,
    _drive: Drive,
    _capabilities: u16,
    size: usize,
}

const SECTOR_SIZE: usize = 2048;

impl<T: ProcessTypes + 'static> ATAPI<T> {
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
            Box::new(ATAPI {
                _controller: controller,
                _channel: channel,
                _drive: drive,
                _capabilities: capabilities,
                size: size,
            }),
        )
    }
}

impl<T: ProcessTypes + 'static> Device for ATAPI<T> {
    fn read(&self, _: usize, _: &mut [u8]) -> base::error::Result<usize> {
        Err(Box::new(ATAPIError::NotImplemented))
    }

    fn write(&mut self, _: usize, _: &[u8]) -> base::error::Result<usize> {
        Err(Box::new(ATAPIError::NotImplemented))
    }

    fn read_register(&mut self, _: usize) -> base::error::Result<usize> {
        Err(Box::new(ATAPIError::NotSupported))
    }

    fn write_register(&mut self, _: usize, _: usize) -> base::error::Result<()> {
        Err(Box::new(ATAPIError::NotSupported))
    }

    fn ioctrl(&mut self, code: usize, _: usize) -> base::error::Result<usize> {
        match code {
            0 => Ok(self.size),
            _ => Err(Box::new(ATAPIError::InvalidIOCtrl)),
        }
    }
}

impl base::error::Error for ATAPIError {
    fn module_number(&self) -> i32 {
        IDE_DRIVER_MODULE_NUMBER
    }

    fn error_number(&self) -> base::error::Status {
        match self {
            ATAPIError::NotImplemented => base::error::Status::NotImplemented,
            ATAPIError::NotSupported => base::error::Status::NotSupported,
            ATAPIError::InvalidIOCtrl => base::error::Status::InvalidIOCtrl,
        }
    }
}

impl core::fmt::Display for ATAPIError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            ATAPIError::NotImplemented => write!(f, "Not implemented for ATAPI yet"),
            ATAPIError::NotSupported => write!(f, "Not supported for ATAPI"),
            ATAPIError::InvalidIOCtrl => write!(f, "Invalid I/O control for ATAPI"),
        }
    }
}
