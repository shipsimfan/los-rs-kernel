use super::Event;
use crate::{device::DeviceBox, error, queue::Queue};

pub struct Console {
    output_device: DeviceBox,
    event_queue: Queue<Event>,
}

pub const CONSOLE_IOCTRL_CLEAR: usize = 0;

impl Console {
    pub fn new(output_device: DeviceBox) -> Self {
        Console {
            output_device,
            event_queue: Queue::new(),
        }
    }

    pub fn write(&mut self, buffer: &[u8]) -> error::Result<()> {
        self.output_device.lock().write(0, buffer)
    }

    pub fn write_str(&mut self, string: &str) -> error::Result<()> {
        self.write(string.as_bytes())
    }

    pub fn clear(&mut self) -> error::Result<()> {
        self.output_device.lock().ioctrl(CONSOLE_IOCTRL_CLEAR, 0)?;
        Ok(())
    }

    pub fn push_event(&mut self, event: Event) {
        self.event_queue.push(event);
    }

    pub fn peek_event(&mut self) -> Option<Event> {
        self.event_queue.pop()
    }
}
