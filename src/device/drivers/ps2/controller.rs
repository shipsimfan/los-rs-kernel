use alloc::{boxed::Box, format, sync::Arc};

use crate::{
    device::{self, inb, outb, Device, DeviceBox},
    error, interrupts,
    locks::Mutex,
    logln, process, session, time,
};

use super::keyboard;

pub struct Controller {
    port_exists: [bool; 2],
    port_irq: [bool; 2],
    initializing: [bool; 2],
    port_data: [u8; 2],
    devices: [Option<DeviceBox>; 2],
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

unsafe fn first_port_irq(context: usize) {
    let controller = &mut *(context as *mut Controller);
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

unsafe fn second_port_irq(context: usize) {
    let controller = unsafe { &mut *(context as *mut Controller) };
    let data = inb(REGISTER_DATA);

    if controller.initializing[1] {
        if !controller.port_irq[1] {
            controller.port_data[1] = data;
        }

        controller.port_irq[1] = true;
    } else {
        match &controller.devices[1] {
            Some(device) => match unsafe { (*device.as_ptr()).ioctrl(0, data as usize) } {
                Ok(_) => {}
                Err(_) => {}
            },
            None => {}
        }
    }
}

impl Controller {
    pub fn new() -> Self {
        Controller {
            port_exists: [false, false],
            port_irq: [true, true],
            initializing: [true, true],
            port_data: [0, 0],
            devices: [None, None],
        }
    }

    pub fn write_and_wait(&mut self, port: usize, data: u8) -> error::Result<()> {
        self.port_irq[port] = false;
        self.write_register(port, data as usize)?;

        let start = time::current_time_millis();
        while !self.port_irq[port] {
            if time::current_time_millis() > start + TIMEOUT {
                return Err(error::Status::TimedOut);
            }
        }

        Ok(())
    }

    pub fn stop_initializing(&mut self, port: usize) {
        self.initializing[port] = false;
    }

    fn enumerate_devices(&mut self) -> error::Result<usize> {
        // Install interrupt handlers
        interrupts::irq::install_irq_handler(1, first_port_irq, self as *mut _ as usize);
        interrupts::irq::install_irq_handler(12, second_port_irq, self as *mut _ as usize);

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
            logln!("\nError during self test");
            return Err(error::Status::IOError);
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

    fn ident_port(&mut self, port: usize) -> error::Result<()> {
        // Reset
        match self.write_and_wait(port, DEVICE_COMMAND_RESET) {
            Ok(()) => {}
            Err(status) => {
                self.port_exists[port] = false;
                return Err(status);
            }
        }

        if self.port_data[port] != DEVICE_ACK {
            self.port_exists[port] = false;
            return Err(error::Status::IOError);
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
            return Err(error::Status::IOError);
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
            let current_session = match process::get_current_thread_mut()
                .get_process_mut()
                .get_session_mut()
            {
                None => match session::get_session_mut(1) {
                    Some(session) => session,
                    None => {
                        logln!("Unable to register keyboard without session!");
                        self.port_exists[port] = false;
                        return Ok(());
                    }
                },
                Some(session) => session,
            };

            let keyboard: DeviceBox = Arc::new(Mutex::new(Box::new(keyboard::Keyboard::new(
                self,
                port,
                current_session,
            )?)));
            let path = format!("/ps2/{}", port);
            device::register_device(&path, keyboard.clone())?;
            self.devices[port] = Some(keyboard);
        } else {
            logln!(
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

impl Device for Controller {
    fn read(&self, _: usize, _: &mut [u8]) -> error::Result<()> {
        Err(error::Status::NotSupported)
    }

    fn write(&mut self, _: usize, _: &[u8]) -> error::Result<()> {
        Err(error::Status::NotSupported)
    }

    fn read_register(&mut self, _: usize) -> error::Result<usize> {
        Err(error::Status::NotSupported)
    }

    fn write_register(&mut self, address: usize, value: usize) -> error::Result<()> {
        if address > 1 {
            return Err(error::Status::BadAddress);
        }

        if !self.port_exists[address] {
            return Err(error::Status::BadAddress);
        }

        if address == 1 {
            write_command(COMMAND_SELECT_SECOND_INPUT);
        }

        let start = time::current_time_millis();
        while inb(REGISTER_STATUS) & 2 != 0 {
            if time::current_time_millis() > start + TIMEOUT {
                return Err(error::Status::TimedOut);
            }
        }

        outb(REGISTER_DATA, value as u8);

        Ok(())
    }

    fn ioctrl(&mut self, code: usize, _: usize) -> error::Result<usize> {
        match code {
            0 => self.enumerate_devices(),
            _ => Err(error::Status::InvalidIOCtrl),
        }
    }
}
