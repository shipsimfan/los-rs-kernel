use super::keyboard;
use alloc::{boxed::Box, format};
use base::{error::PS2_DRIVER_MODULE_NUMBER, log_error, multi_owner::Owner};
use device::{inb, outb, Device};
use process::{Mutex, ProcessTypes};
use sessions::Session;

#[derive(Debug)]
enum ControllerError {
    TimedOut,
    IOError,
    NotSupported,
    InvalidIOCtrl,
    BadAddress,
}

pub struct Controller<T: ProcessTypes<Owner = Box<dyn Session<T>>> + 'static> {
    port_exists: [bool; 2],
    port_irq: [bool; 2],
    initializing: [bool; 2],
    port_data: [u8; 2],
    devices: [Option<Owner<Box<dyn Device>, Mutex<Box<dyn Device>, T>>>; 2],
}

pub const REGISTER_DATA: u16 = 0x60;
const REGISTER_COMMAND: u16 = 0x64;
const REGISTER_STATUS: u16 = 0x64;

const COMMAND_READ_CONFIG_BYTE: u8 = 0x20;
const COMMAND_WRITE_CONFIG_BYTE: u8 = 0x60;
const COMMAND_DISABLE_SECOND_PORT: u8 = 0xA7;
const COMMAND_ENABLE_SECOND_PORT: u8 = 0xA8;
const COMMAND_TEST_SECOND_PORT: u8 = 0xA9;
const COMMAND_TEST: u8 = 0xAA;
const COMMAND_TEST_FIRST_PORT: u8 = 0xAB;
const COMMAND_DISABLE_FIRST_PORT: u8 = 0xAD;
const COMMAND_ENABLE_FIRST_PORT: u8 = 0xAE;
const COMMAND_SELECT_SECOND_INPUT: u8 = 0x04;

const DEVICE_COMMAND_IDENTIFY: u8 = 0xF2;
pub const DEVICE_COMMAND_ENABLE_SCAN: u8 = 0xF4;
const DEVICE_COMMAND_DISABLE_SCAN: u8 = 0xF5;
const DEVICE_COMMAND_RESET: u8 = 0xFF;

const DEVICE_ACK: u8 = 0xFA;

const TIMEOUT: usize = 10;

#[inline]
fn write_command(command: u8) {
    while inb(REGISTER_STATUS) & 2 != 0 {}
    outb(REGISTER_COMMAND, command);
}

#[inline]
fn read_data() -> u8 {
    while inb(REGISTER_STATUS) & 1 == 0 {}
    inb(REGISTER_DATA)
}

#[inline]
fn write_data(value: u8) {
    while inb(REGISTER_STATUS) & 2 != 0 {}
    outb(REGISTER_DATA, value);
}

unsafe fn first_port_irq<T: ProcessTypes<Owner = Box<dyn Session<T>>> + 'static>(context: usize) {
    let controller = &mut *(context as *mut Controller<T>);
    let data = inb(REGISTER_DATA);

    if controller.initializing[0] {
        if !controller.port_irq[0] {
            controller.port_data[0] = data;
        }

        controller.port_irq[0] = true;
    } else {
        match &controller.devices[0] {
            Some(device) => match (*device.as_ptr()).ioctrl(0, data as usize) {
                Ok(_) => {}
                Err(_) => {}
            },
            None => {}
        }
    }
}

unsafe fn second_port_irq<T: ProcessTypes<Owner = Box<dyn Session<T>>> + 'static>(context: usize) {
    let controller = &mut *(context as *mut Controller<T>);
    let data = inb(REGISTER_DATA);

    if controller.initializing[1] {
        if !controller.port_irq[1] {
            controller.port_data[1] = data;
        }

        controller.port_irq[1] = true;
    } else {
        match &controller.devices[1] {
            Some(device) => match (*device.as_ptr()).ioctrl(0, data as usize) {
                Ok(_) => {}
                Err(_) => {}
            },
            None => {}
        }
    }
}

impl<T: ProcessTypes<Owner = Box<dyn Session<T>>>> Controller<T> {
    pub fn new() -> Owner<Box<dyn Device>, Mutex<Box<dyn Device>, T>> {
        Owner::new(Box::new(Controller::<T> {
            port_exists: [false, false],
            port_irq: [true, true],
            initializing: [true, true],
            port_data: [0, 0],
            devices: [None, None],
        }) as Box<dyn Device>)
    }

    pub fn write_and_wait(&mut self, port: usize, data: u8) -> base::error::Result<()> {
        self.port_irq[port] = false;
        self.write_register(port, data as usize)?;

        let start = time::current_time_millis();
        while !self.port_irq[port] {
            if time::current_time_millis() > start + TIMEOUT {
                return Err(Box::new(ControllerError::TimedOut));
            }
        }

        Ok(())
    }

    pub fn stop_initializing(&mut self, port: usize) {
        self.initializing[port] = false;
    }

    fn enumerate_devices(&mut self) -> base::error::Result<usize> {
        // Install interrupt handlers
        interrupts::irqs::install_irq_handler(1, first_port_irq::<T>, self as *mut _ as usize);
        interrupts::irqs::install_irq_handler(12, second_port_irq::<T>, self as *mut _ as usize);

        // Disable device for initialization
        write_command(COMMAND_DISABLE_FIRST_PORT);
        write_command(COMMAND_DISABLE_SECOND_PORT);

        // Flush data buffer
        inb(REGISTER_DATA);

        // Set the configuration byte
        write_command(COMMAND_READ_CONFIG_BYTE);
        let mut config = read_data();

        config &= 0b10111100;
        let has_two_ports = config & 5 != 0;

        write_command(COMMAND_WRITE_CONFIG_BYTE);
        write_data(config);

        // Perform self test
        write_command(COMMAND_TEST);
        let response = read_data();
        if response != 0x55 {
            log_error!("\nError during self test");
            return Err(Box::new(ControllerError::IOError));
        }

        // Rewrite configuration byte
        write_command(COMMAND_WRITE_CONFIG_BYTE);
        write_data(config);

        // Test for two channels
        let has_two_ports = if has_two_ports {
            write_command(COMMAND_ENABLE_SECOND_PORT);
            write_command(COMMAND_READ_CONFIG_BYTE);
            let config = read_data();

            if config & 5 != 0 {
                false
            } else {
                write_command(COMMAND_DISABLE_SECOND_PORT);
                true
            }
        } else {
            false
        };

        // Test interfaces
        write_command(COMMAND_TEST_FIRST_PORT);
        let status = read_data();
        self.port_exists[0] = status == 0;

        if has_two_ports {
            write_command(COMMAND_TEST_SECOND_PORT);
            let status = read_data();
            self.port_exists[1] = status == 0;
        }

        // Enable devices
        let mut int_pins = 0;
        if self.port_exists[0] {
            write_command(COMMAND_ENABLE_FIRST_PORT);
            int_pins |= 1;
        }

        if self.port_exists[1] {
            write_command(COMMAND_ENABLE_SECOND_PORT);
            int_pins |= 2;
        }

        write_command(COMMAND_READ_CONFIG_BYTE);
        let mut config = read_data();
        config |= int_pins;
        write_command(COMMAND_WRITE_CONFIG_BYTE);
        write_data(config);

        if self.port_exists[0] {
            self.ident_port(0)?;
        }

        if self.port_exists[1] {
            self.ident_port(1)?;
        }

        Ok(0)
    }

    fn ident_port(&mut self, port: usize) -> base::error::Result<()> {
        // Reset
        match self.write_and_wait(port, DEVICE_COMMAND_RESET) {
            Ok(()) => {}
            Err(error) => {
                self.port_exists[port] = false;
                return Err(error);
            }
        }

        if self.port_data[port] != DEVICE_ACK {
            self.port_exists[port] = false;
            return Err(Box::new(ControllerError::IOError));
        }

        // Identify device
        match self.write_and_wait(port, DEVICE_COMMAND_DISABLE_SCAN) {
            Ok(()) => {}
            Err(status) => {
                self.port_exists[port] = false;
                return Err(status);
            }
        }

        if self.port_data[port] != DEVICE_ACK {}

        let mut ident = [0, 0];
        let mut len = -1;
        match self.write_and_wait(port, DEVICE_COMMAND_IDENTIFY) {
            Ok(()) => {}
            Err(status) => {
                self.port_exists[port] = false;
                return Err(status);
            }
        }

        if self.port_data[port] != DEVICE_ACK {
            self.port_exists[port] = false;
            return Err(Box::new(ControllerError::IOError));
        }

        self.port_irq[port] = false;
        let start = time::current_time_millis();
        while !self.port_irq[port] {
            if time::current_time_millis() > start + TIMEOUT {
                len = 0;
                break;
            }
        }

        if len == -1 {
            ident[0] = self.port_data[port];
            self.port_irq[port] = false;
            let start = time::current_time_millis();
            while !self.port_irq[port] {
                if time::current_time_millis() > start + TIMEOUT {
                    len = 1;
                    break;
                }
            }

            if len == -1 {
                ident[1] = self.port_data[port];
                len = 2;
            }
        }

        if len == 0 {
            let current_session = match sessions::get_session::<T>(1) {
                Some(session) => session,
                None => {
                    log_error!("Unable to register keyboard without session!");
                    self.port_exists[port] = false;
                    return Ok(());
                }
            };

            let keyboard = keyboard::Keyboard::new(self, port, current_session)?;
            let path = format!("/ps2/{}", port);
            device::register_device(&path, keyboard.clone())?;
            self.devices[port] = Some(keyboard);
        } else {
            log_error!(
                "Unknown device on port {} - {:#X}, {:#X}",
                port,
                ident[0],
                ident[1]
            );
            self.port_exists[port] = false;
        }

        Ok(())
    }
}

impl<T: ProcessTypes<Owner = Box<dyn Session<T>>>> Device for Controller<T> {
    fn read(&self, _: usize, _: &mut [u8]) -> base::error::Result<usize> {
        Err(Box::new(ControllerError::NotSupported))
    }

    fn write(&mut self, _: usize, _: &[u8]) -> base::error::Result<usize> {
        Err(Box::new(ControllerError::NotSupported))
    }

    fn read_register(&mut self, _: usize) -> base::error::Result<usize> {
        Err(Box::new(ControllerError::NotSupported))
    }

    fn write_register(&mut self, address: usize, value: usize) -> base::error::Result<()> {
        if address > 1 {
            return Err(Box::new(ControllerError::BadAddress));
        }

        if !self.port_exists[address] {
            return Err(Box::new(ControllerError::BadAddress));
        }

        if address == 1 {
            write_command(COMMAND_SELECT_SECOND_INPUT);
        }

        let start = time::current_time_millis();
        while inb(REGISTER_STATUS) & 2 != 0 {
            if time::current_time_millis() > start + TIMEOUT {
                return Err(Box::new(ControllerError::TimedOut));
            }
        }

        outb(REGISTER_DATA, value as u8);

        Ok(())
    }

    fn ioctrl(&mut self, code: usize, _: usize) -> base::error::Result<usize> {
        match code {
            0 => self.enumerate_devices(),
            _ => Err(Box::new(ControllerError::InvalidIOCtrl)),
        }
    }
}

impl base::error::Error for ControllerError {
    fn module_number(&self) -> i32 {
        PS2_DRIVER_MODULE_NUMBER
    }

    fn error_number(&self) -> base::error::Status {
        match self {
            ControllerError::TimedOut => base::error::Status::TimedOut,
            ControllerError::BadAddress => base::error::Status::BadAddress,
            ControllerError::IOError => base::error::Status::IOError,
            ControllerError::InvalidIOCtrl => base::error::Status::InvalidIOCtrl,
            ControllerError::NotSupported => base::error::Status::NotSupported,
        }
    }
}

impl core::fmt::Display for ControllerError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            ControllerError::TimedOut => write!(f, "PS/2 controller timed-out"),
            ControllerError::BadAddress => write!(f, "Bad address provided"),
            ControllerError::IOError => write!(f, "I/O error with PS/2 controller"),
            ControllerError::InvalidIOCtrl => write!(f, "Invalid I/O control for PS/2 controller"),
            ControllerError::NotSupported => write!(f, "Not supported for PS/2 controller"),
        }
    }
}
