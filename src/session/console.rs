use crate::{device::DeviceBox, error};

pub struct Console {
    output_device: DeviceBox,
}

impl Console {
    pub fn new(output_device: DeviceBox) -> Self {
        Console { output_device }
    }

    pub fn write(&mut self, buffer: &[u8]) -> Result<(), error::Status> {
        self.output_device.lock().write(0, buffer)
    }

    pub fn write_str(&mut self, string: &str) -> Result<(), error::Status> {
        self.write(string.as_bytes())
    }
}
