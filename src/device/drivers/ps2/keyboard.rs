use crate::{
    device::{outb, Device},
    error, logln,
};

use super::controller;

pub struct Keyboard {
    caps_lock: bool,
    num_lock: bool,
    scroll_lock: bool,
    left_shift: bool,
    right_shift: bool,
    ignore_next_irq: bool,
}

impl Keyboard {
    pub fn new(
        controller: &mut controller::Controller,
        port: usize,
    ) -> Result<Self, error::Status> {
        // Set scancode set to 1
        controller.write_and_wait(port, 0xF0)?;

        outb(controller::REGISTER_DATA, 1);

        // Enable sdevicecanning
        controller.write_and_wait(port, controller::DEVICE_COMMAND_ENABLE_SCAN)?;
        controller.stop_initializing(port);

        // Return keyboard
        Ok(Keyboard {
            caps_lock: false,
            num_lock: false,
            scroll_lock: false,
            left_shift: false,
            right_shift: false,
            ignore_next_irq: false,
        })
    }

    fn irq(&mut self, data: u8) {
        if self.ignore_next_irq {
            self.ignore_next_irq = false;
        } else {
            logln!("Key: {}", data);
        }
    }
}

impl Device for Keyboard {
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

    fn ioctrl(&mut self, code: usize, argument: usize) -> Result<usize, error::Status> {
        match code {
            0 => {
                self.irq(argument as u8);
                Ok(0)
            }
            _ => Err(error::Status::NotSupported),
        }
    }
}
