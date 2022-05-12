use crate::{Event, Session};
use alloc::boxed::Box;
use base::{
    map::{Map, Mappable, INVALID_ID},
    multi_owner::{Owner, Reference},
    queue::Queue,
};
use device::Device;
use process::{CurrentQueue, Mutex, Process, ProcessOwner, ProcessTypes, ThreadQueue};

pub struct ConsoleSession<T: ProcessTypes + 'static> {
    processes: Map<Reference<Process<T>>>,
    id: isize,

    output_device: Reference<Box<dyn Device>, Mutex<Box<dyn Device>, T>>,

    event_queue: Queue<Event>,
    event_thread_queue: ThreadQueue<T>,
}

pub struct ConsoleOutputDevice<T: ProcessTypes + 'static> {
    output_device: Reference<Box<dyn Device>, Mutex<Box<dyn Device>, T>>,
}

#[derive(Debug, Clone)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

#[derive(Debug)]
struct NoDeviceError;

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

impl<T: ProcessTypes> ConsoleSession<T> {
    pub fn new(
        output_device: Reference<Box<dyn Device>, Mutex<Box<dyn Device>, T>>,
    ) -> base::error::Result<Owner<Box<dyn Session<T>>, Mutex<Box<dyn Session<T>>, T>>> {
        //output_device.lock().ioctrl(IOCTRL_CLEAR, 0)?;

        let session = Box::new(ConsoleSession {
            processes: Map::new(),
            id: INVALID_ID,
            output_device,
            event_queue: Queue::new(),
            event_thread_queue: ThreadQueue::new(),
        }) as Box<dyn Session<T>>;

        Ok(Owner::new(session))
    }

    pub fn get_output_device(&self) -> ConsoleOutputDevice<T> {
        ConsoleOutputDevice {
            output_device: self.output_device.clone(),
        }
    }
}

impl<T: ProcessTypes> Session<T> for ConsoleSession<T> {
    fn push_event(&mut self, event: Event) {
        match self.event_thread_queue.pop() {
            Some(thread) => process::queue_thread(thread),
            None => {}
        }

        self.event_queue.push(event);
    }

    fn peek_event(&mut self) -> Option<Event> {
        unsafe {
            base::critical::enter_local();
            let res = self.event_queue.pop();
            base::critical::leave_local();
            res
        }
    }

    fn get_event_thread_queue(&self) -> Option<CurrentQueue<T>> {
        Some(self.event_thread_queue.current_queue())
    }

    fn as_console(&mut self) -> Option<&mut ConsoleSession<T>> {
        Some(self)
    }
}

impl<T: ProcessTypes> ProcessOwner<T> for ConsoleSession<T> {
    fn insert_process(&mut self, process: Reference<Process<T>>) {
        self.processes.insert(process);
    }

    fn remove_process(&mut self, id: isize) {
        self.processes.remove(id);
    }
}

impl<T: ProcessTypes> Mappable for ConsoleSession<T> {
    fn set_id(&mut self, id: isize) {
        self.id = id;
    }

    fn id(&self) -> isize {
        self.id
    }
}

unsafe impl<T: ProcessTypes> Send for ConsoleSession<T> {}

impl<T: ProcessTypes> ConsoleOutputDevice<T> {
    pub fn write(&mut self, buffer: &[u8]) -> base::error::Result<usize> {
        self.output_device
            .lock(|device| device.write(0, buffer))
            .unwrap_or(Err(NoDeviceError::new()))
    }

    pub fn write_str(&mut self, string: &str) -> base::error::Result<usize> {
        self.write(string.as_bytes())
    }

    pub fn clear(&mut self) -> base::error::Result<()> {
        self.output_device
            .lock(|device| device.ioctrl(IOCTRL_CLEAR, 0))
            .unwrap_or(Err(NoDeviceError::new()))?;
        Ok(())
    }

    pub fn set_attribute(&mut self, attribute: usize) -> base::error::Result<()> {
        self.output_device
            .lock(|device| device.ioctrl(IOCTRL_SET_ATTRIBUTE, attribute))
            .unwrap_or(Err(NoDeviceError::new()))?;
        Ok(())
    }

    pub fn set_foreground_color(&mut self, color: Color) -> base::error::Result<()> {
        self.output_device
            .lock(|device| device.ioctrl(IOCTRL_SET_FOREGROUND_COLOR, color.as_usize()))
            .unwrap_or(Err(NoDeviceError::new()))?;
        Ok(())
    }

    pub fn set_background_color(&mut self, color: Color) -> base::error::Result<()> {
        self.output_device
            .lock(|device| device.ioctrl(IOCTRL_SET_BACKGROUND_COLOR, color.as_usize()))
            .unwrap_or(Err(NoDeviceError::new()))?;
        Ok(())
    }

    pub fn set_cursor_pos(&mut self, x: usize, y: usize) -> base::error::Result<()> {
        self.output_device
            .lock(|device| match device.ioctrl(IOCTRL_SET_CURSOR_X, x) {
                Ok(_) => device.ioctrl(IOCTRL_SET_CURSOR_Y, y),
                Err(err) => Err(err),
            })
            .unwrap_or(Err(NoDeviceError::new()))?;
        Ok(())
    }

    pub fn set_cursor_state(&mut self, state: bool) -> base::error::Result<()> {
        self.output_device
            .lock(|device| device.ioctrl(IOCTRL_SET_CURSOR_STATE, if state { 1 } else { 0 }))
            .unwrap_or(Err(NoDeviceError::new()))?;
        Ok(())
    }

    pub fn get_width(&mut self) -> base::error::Result<isize> {
        Ok(self
            .output_device
            .lock(|device| device.ioctrl(IOCTRL_GET_WIDTH, 0))
            .unwrap_or(Err(NoDeviceError::new()))? as isize)
    }

    pub fn get_height(&mut self) -> base::error::Result<isize> {
        Ok(self
            .output_device
            .lock(|device| device.ioctrl(IOCTRL_GET_HEIGHT, 0))
            .unwrap_or(Err(NoDeviceError::new()))? as isize)
    }
}

impl NoDeviceError {
    pub fn new() -> Box<dyn base::error::Error> {
        Box::new(NoDeviceError)
    }
}

impl base::error::Error for NoDeviceError {
    fn module_number(&self) -> i32 {
        base::error::SESSION_MODULE_NUMBER
    }

    fn error_number(&self) -> base::error::Status {
        base::error::Status::NoDevice
    }
}

impl core::fmt::Display for NoDeviceError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "No device")
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
