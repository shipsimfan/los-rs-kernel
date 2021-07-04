use super::{constants::*, controller::IDEController};
use crate::{
    device::{self, Device},
    error,
    locks::Mutex,
    logln,
};
use alloc::{boxed::Box, format, string::String, sync::Arc};

pub struct ATAPI {
    channel: Channel,
    drive: Drive,
    signature: u16,
    capabilities: u16,
    command_sets: u32,
    size: usize,
    model: String,
}

impl ATAPI {
    pub fn create(
        controller: &mut IDEController,
        channel: Channel,
        drive: Drive,
        signature: u16,
        capabilities: u16,
        command_sets: u32,
        size: usize,
        model: String,
    ) -> error::Result {
        let path = format!("/ide/{}_{}", channel, drive);

        device::register_device(
            &path,
            Arc::new(Mutex::new(Box::new(ATAPI {
                channel: channel,
                drive: drive,
                signature: signature,
                capabilities: capabilities,
                command_sets: command_sets,
                size: size,
                model: model,
            }))),
        )
    }
}

impl Device for ATAPI {
    fn read(&self, _: usize, _: &mut [u8]) -> error::Result {
        Err(error::Status::NotSupported)
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
