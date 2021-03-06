use super::Event;
use crate::{
    device::DeviceReference,
    error,
    process::{self, CurrentQueue, ThreadQueue},
    queue::Queue,
};

pub struct Console {
    output_device: DeviceReference,
    event_queue: Queue<Event>,
    event_thread_queue: ThreadQueue,
}

pub struct ConsoleOutputDevice {
    output_device: DeviceReference,
}

#[derive(Debug, Clone)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

#[allow(dead_code)]
pub const STYLE_RESET: usize = 0;
#[allow(dead_code)]
pub const STYLE_BOLD: usize = 1;
pub const STYLE_DIM: usize = 2;
pub const STYLE_UNDERLINE: usize = 4;
pub const STYLE_STRIKETRHOUGH: usize = 8;

pub const IOCTRL_CLEAR: usize = 0;
pub const IOCTRL_SET_ATTRIBUTE: usize = 1;
pub const IOCTRL_SET_FOREGROUND_COLOR: usize = 2;
pub const IOCTRL_SET_BACKGROUND_COLOR: usize = 3;
pub const IOCTRL_SET_CURSOR_X: usize = 4;
pub const IOCTRL_SET_CURSOR_Y: usize = 5;
pub const IOCTRL_GET_WIDTH: usize = 6;
pub const IOCTRL_GET_HEIGHT: usize = 7;
pub const IOCTRL_SET_CURSOR_STATE: usize = 8;

impl Console {
    pub fn new(output_device: DeviceReference) -> error::Result<Self> {
        //output_device.lock().ioctrl(IOCTRL_CLEAR, 0)?;

        Ok(Console {
            output_device,
            event_queue: Queue::new(),
            event_thread_queue: ThreadQueue::new(),
        })
    }

    pub fn push_event(&mut self, event: Event) {
        match self.event_thread_queue.pop() {
            Some(thread) => process::queue_thread(thread),
            None => {}
        }

        self.event_queue.push(event);
    }

    pub fn peek_event(&mut self) -> Option<Event> {
        unsafe {
            crate::critical::enter_local();
            let res = self.event_queue.pop();
            crate::critical::leave_local();
            res
        }
    }

    pub fn get_event_thread_queue(&self) -> CurrentQueue {
        self.event_thread_queue.into_current_queue()
    }

    pub fn get_output_device(&self) -> ConsoleOutputDevice {
        ConsoleOutputDevice {
            output_device: self.output_device.clone(),
        }
    }
}

impl ConsoleOutputDevice {
    pub fn write(&mut self, buffer: &[u8]) -> error::Result<()> {
        self.output_device.lock().write(0, buffer)
    }

    pub fn write_str(&mut self, string: &str) -> error::Result<()> {
        self.write(string.as_bytes())
    }

    pub fn clear(&mut self) -> error::Result<()> {
        self.output_device.lock().ioctrl(IOCTRL_CLEAR, 0)?;
        Ok(())
    }

    pub fn set_attribute(&mut self, attribute: usize) -> error::Result<()> {
        self.output_device
            .lock()
            .ioctrl(IOCTRL_SET_ATTRIBUTE, attribute)?;
        Ok(())
    }

    pub fn set_foreground_color(&mut self, color: Color) -> error::Result<()> {
        self.output_device
            .lock()
            .ioctrl(IOCTRL_SET_FOREGROUND_COLOR, color.as_usize())?;
        Ok(())
    }

    pub fn set_background_color(&mut self, color: Color) -> error::Result<()> {
        self.output_device
            .lock()
            .ioctrl(IOCTRL_SET_BACKGROUND_COLOR, color.as_usize())?;
        Ok(())
    }

    pub fn set_cursor_pos(&mut self, x: usize, y: usize) -> error::Result<()> {
        let mut output_device = self.output_device.lock();
        output_device.ioctrl(IOCTRL_SET_CURSOR_X, x)?;
        output_device.ioctrl(IOCTRL_SET_CURSOR_Y, y)?;
        Ok(())
    }

    pub fn set_cursor_state(&mut self, state: bool) -> error::Result<()> {
        self.output_device
            .lock()
            .ioctrl(IOCTRL_SET_CURSOR_STATE, if state { 1 } else { 0 })?;
        Ok(())
    }

    pub fn get_width(&mut self) -> error::Result<isize> {
        Ok(self.output_device.lock().ioctrl(IOCTRL_GET_WIDTH, 0)? as isize)
    }

    pub fn get_height(&mut self) -> error::Result<isize> {
        Ok(self.output_device.lock().ioctrl(IOCTRL_GET_HEIGHT, 0)? as isize)
    }
}

impl Color {
    #[inline]
    pub const fn new(red: u8, green: u8, blue: u8) -> Self {
        Color { red, green, blue }
    }

    #[inline]
    pub fn from_usize(val: usize) -> Self {
        Color {
            red: (val.wrapping_shr(16) & 0xFF) as u8,
            green: (val.wrapping_shr(8) & 0xFF) as u8,
            blue: (val.wrapping_shr(0) & 0xFF) as u8,
        }
    }

    #[inline]
    pub fn average(color1: &Color, color2: &Color) -> Self {
        Color {
            red: (((color1.red as usize) + (color2.red as usize)) / 2) as u8,
            green: (((color1.green as usize) + (color2.green as usize)) / 2) as u8,
            blue: (((color1.blue as usize) + (color2.blue as usize)) / 2) as u8,
        }
    }

    #[inline]
    pub fn as_usize(&self) -> usize {
        (self.blue as usize) | ((self.green as usize) << 8) | ((self.red as usize) << 16)
    }
}
