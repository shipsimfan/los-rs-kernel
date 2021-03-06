use alloc::string::String;

use super::{ata::ATA, atapi::ATAPI, constants::*};
use crate::{
    device::{self, inb, outb, Device},
    error, time,
};

struct ChannelRegisters {
    pub io: u16,
    pub control: u16,
    pub bus_master: u16,
    pub n_ien: u8,
}

pub struct IDEController {
    channels: [ChannelRegisters; 2],
}

pub const IOCTRL_ENUMERATE: usize = 0;
pub const IOCTRL_POLL: usize = 1;
pub const IOCTRL_ADVANCED_POLL: usize = 2;
pub const IOCTRL_SET_CHANNEL_INTERRUPT: usize = 3;
pub const IOCTRL_CLEAR_CHANNEL_INTERRUPT: usize = 4;

unsafe fn irq_handler(context: usize) {
    let _controller = &mut *(context as *mut IDEController);
}

impl IDEController {
    pub fn new(bar0: usize, bar1: usize, bar2: usize, bar3: usize, bar4: usize) -> Self {
        IDEController {
            channels: [
                ChannelRegisters {
                    io: ((bar0 & 0xFFFFFFFC) + 0x1F0 * if bar0 == 0 { 1 } else { 0 }) as u16,
                    control: ((bar1 & 0xFFFFFFFC) + 0x3F6 * if bar1 == 0 { 1 } else { 0 }) as u16,
                    bus_master: ((bar4 & 0xFFFFFFFC) + 0) as u16,
                    n_ien: 2,
                },
                ChannelRegisters {
                    io: ((bar2 & 0xFFFFFFFC) + 0x170 * if bar2 == 0 { 1 } else { 0 }) as u16,
                    control: ((bar3 & 0xFFFFFFFC) + 0x376 * if bar3 == 0 { 1 } else { 0 }) as u16,
                    bus_master: ((bar4 & 0xFFFFFFFC) + 8) as u16,
                    n_ien: 2,
                },
            ],
        }
    }

    fn enumerate_drives(&mut self) -> error::Result<usize> {
        // Install IRQ handlers
        crate::interrupts::irq::install_irq_handler(
            14,
            irq_handler,
            self as *mut IDEController as usize,
        );
        crate::interrupts::irq::install_irq_handler(
            15,
            irq_handler,
            self as *mut IDEController as usize,
        );

        // Disable IRQs
        self.write_register(REGISTER_CONTROL, 2)?;
        self.write_register(0x100 | REGISTER_CONTROL, 2)?;

        // Enumerate drives
        for i in 0..2 {
            let channel = Channel::from(i);
            for j in 0..2 {
                let drive = Drive::from(j);
                let mut err = 0;
                let mut drive_type = DRIVE_TYPE_ATA;

                // Select drive
                self.write_register(channel.reg(REGISTER_DRIVE_SELECT), 0xA0 | drive.select())?;
                time::sleep(2);

                // Send ATA identify command
                self.write_register(channel.reg(REGISTER_COMMAND), COMMAND_IDENTIFY)?;
                time::sleep(2);

                // Polling
                if self.read_register(channel.reg(REGISTER_STATUS))? == 0 {
                    continue;
                }

                loop {
                    let status = self.read_register(channel.reg(REGISTER_STATUS))?;
                    if status & STATUS_ERROR != 0 {
                        err = 1;
                        break;
                    }

                    if status & STATUS_BUSY == 0 && status & STATUS_DATA_REQUEST_READY != 0 {
                        break;
                    }
                }

                // Probe for ATAPI devices
                if err != 0 {
                    let cl = self.read_register(channel.reg(REGISTER_LBA_1))?;
                    let ch = self.read_register(channel.reg(REGISTER_LBA_2))?;

                    if cl == 0x14 && ch == 0xEB {
                        drive_type = DRIVE_TYPE_ATAPI;
                    } else if cl == 0x69 && ch == 0x96 {
                        drive_type = DRIVE_TYPE_ATAPI;
                    } else {
                        continue;
                    }

                    self.write_register(channel.reg(REGISTER_COMMAND), COMMAND_IDENTIFY_PACKET)?;
                    time::sleep(2);
                }

                // Read identification space
                let mut ident = [0u8; 512];
                self.read_buffer(channel.clone(), REGISTER_DATA as u16, &mut ident)?;

                // Read device parameters
                let capabilities = (ident[IDENT_CAPABILITIES] as u16)
                    | ((ident[IDENT_CAPABILITIES + 1] as u16) << 8);
                let command_sets = (ident[IDENT_COMMAND_SETS] as u32)
                    | ((ident[IDENT_COMMAND_SETS + 1] as u32) << 8)
                    | ((ident[IDENT_COMMAND_SETS + 2] as u32) << 16)
                    | ((ident[IDENT_COMMAND_SETS + 3] as u32) << 24);

                let size = if command_sets & (1 << 26) != 0 {
                    (ident[IDENT_MAX_LBA_EXT] as usize)
                        | ((ident[IDENT_MAX_LBA_EXT + 1] as usize) << 8)
                        | ((ident[IDENT_MAX_LBA_EXT + 2] as usize) << 16)
                        | ((ident[IDENT_MAX_LBA_EXT + 3] as usize) << 24)
                        | ((ident[IDENT_MAX_LBA_EXT + 4] as usize) << 32)
                        | ((ident[IDENT_MAX_LBA_EXT + 5] as usize) << 40)
                } else {
                    (ident[IDENT_MAX_LBA] as usize)
                        | ((ident[IDENT_MAX_LBA + 1] as usize) << 8)
                        | ((ident[IDENT_MAX_LBA + 2] as usize) << 16)
                        | ((ident[IDENT_MAX_LBA + 3] as usize) << 24)
                };

                let mut i = 0;
                let mut model = String::with_capacity(40);
                while i < 40 {
                    model.push(ident[IDENT_MODEL + i + 1] as char);
                    model.push(ident[IDENT_MODEL + i] as char);

                    i += 2;
                }

                if drive_type == DRIVE_TYPE_ATA {
                    ATA::create(channel.clone(), drive, capabilities, size, model)?;
                } else {
                    ATAPI::create(channel.clone(), drive, capabilities, size, model)?;
                }
            }
        }

        Ok(0)
    }

    fn polling(&mut self, channel: u8, advanced_check: bool) -> error::Result<usize> {
        let channel = (channel as usize) << 8;

        let mut i = 0;
        while i < 4 {
            self.read_register(channel | REGISTER_ALT_STATUS)?;
            i += 1;
        }

        while self.read_register(channel | REGISTER_STATUS)? & STATUS_BUSY != 0 {}

        if advanced_check {
            let state = self.read_register(channel | REGISTER_STATUS)?;
            if state & STATUS_ERROR != 0 {
                Ok(2)
            } else if state & STATUS_DRIVE_FAULT != 0 {
                Ok(1)
            } else if state & STATUS_DATA_REQUEST_READY == 0 {
                Ok(3)
            } else {
                Ok(0)
            }
        } else {
            Ok(0)
        }
    }

    fn read_buffer(&self, channel: Channel, register: u16, buffer: &mut [u8]) -> error::Result<()> {
        let channel = channel as usize;

        if register > 0x07 && register < 0x0C {
            outb(
                self.channels[channel].control + (REGISTER_CONTROL as u16) - 0x0A,
                0x80 | self.channels[channel].n_ien,
            );
        }

        let port = if register < 0x08 {
            self.channels[channel].io + register - 0x00
        } else if register < 0x0C {
            self.channels[channel].io + register - 0x06
        } else if register < 0x0E {
            self.channels[channel].control + register - 0x0A
        } else if register < 0x16 {
            self.channels[channel].bus_master + register - 0x0E
        } else {
            return Err(error::Status::BadAddress);
        };

        let mut i = 0;
        let top = buffer.len();
        while i < top {
            let value = device::ind(port);

            buffer[i + 0] = (value.wrapping_shr(0) & 0xFF) as u8;
            buffer[i + 1] = (value.wrapping_shr(8) & 0xFF) as u8;
            buffer[i + 2] = (value.wrapping_shr(16) & 0xFF) as u8;
            buffer[i + 3] = (value.wrapping_shr(24) & 0xFF) as u8;

            i += 4;
        }

        if register > 0x07 && register < 0x0C {
            outb(
                self.channels[channel].control + (REGISTER_CONTROL as u16) - 0x0A,
                self.channels[channel].n_ien,
            );
        }

        Ok(())
    }

    fn write_buffer(&self, channel: Channel, register: u16, buffer: &[u8]) -> error::Result<()> {
        let channel = channel as usize;

        if register > 0x07 && register < 0x0C {
            outb(
                self.channels[channel].control + (REGISTER_CONTROL as u16) - 0x0A,
                0x80 | self.channels[channel].n_ien,
            );
        }

        let port = if register < 0x08 {
            self.channels[channel].io + register - 0x00
        } else if register < 0x0C {
            self.channels[channel].io + register - 0x06
        } else if register < 0x0E {
            self.channels[channel].control + register - 0x0A
        } else if register < 0x16 {
            self.channels[channel].bus_master + register - 0x0E
        } else {
            return Err(error::Status::BadAddress);
        };

        let mut i = 0;
        let top = buffer.len();
        while i < top {
            let value = (buffer[i + 0] as u32)
                | ((buffer[i + 1] as u32) << 8)
                | ((buffer[i + 2] as u32) << 16)
                | ((buffer[i + 3] as u32) << 24);

            device::outd(port, value);

            i += 4;
        }

        if register > 0x07 && register < 0x0C {
            outb(
                self.channels[channel].control + (REGISTER_CONTROL as u16) - 0x0A,
                self.channels[channel].n_ien,
            );
        }

        Ok(())
    }
}

impl Device for IDEController {
    fn read(&self, address: usize, buffer: &mut [u8]) -> error::Result<()> {
        let register: u16 = (address & 0xFF) as u16;
        let channel = Channel::from(address.wrapping_shr(8) & 1);

        self.read_buffer(channel, register, buffer)
    }

    fn write(&mut self, address: usize, buffer: &[u8]) -> error::Result<()> {
        let register: u16 = (address & 0xFF) as u16;
        let channel = Channel::from(address.wrapping_shr(8) & 1);

        self.write_buffer(channel, register, buffer)
    }

    fn read_register(&mut self, address: usize) -> error::Result<usize> {
        let reg: u16 = (address & 0xFF) as u16;
        let channel = address.wrapping_shr(8) & 1;

        if reg > 0x07 && reg < 0x0C {
            outb(
                self.channels[channel].control + (REGISTER_CONTROL as u16) - 0x0A,
                0x80 | self.channels[channel].n_ien,
            );
        }

        let result = inb(if reg < 0x08 {
            self.channels[channel].io + reg - 0x00
        } else if reg < 0x0C {
            self.channels[channel].io + reg - 0x06
        } else if reg < 0x0E {
            self.channels[channel].control + reg - 0x0A
        } else if reg < 0x16 {
            self.channels[channel].bus_master + reg - 0x0E
        } else {
            return Err(error::Status::BadAddress);
        });

        if reg > 0x07 && reg < 0x0C {
            outb(
                self.channels[channel].control + (REGISTER_CONTROL as u16) - 0x0A,
                self.channels[channel].n_ien,
            );
        }

        Ok(result as usize)
    }

    fn write_register(&mut self, address: usize, value: usize) -> error::Result<()> {
        let reg: u16 = (address & 0xFF) as u16;
        let channel = address.wrapping_shr(8) & 1;
        let value: u8 = (value & 0xFF) as u8;

        if reg > 0x07 && reg < 0x0C {
            outb(
                self.channels[channel].control + (REGISTER_CONTROL as u16) - 0x0A,
                0x80 | self.channels[channel].n_ien,
            );
        }

        outb(
            if reg < 0x08 {
                self.channels[channel].io + reg - 0x00
            } else if reg < 0x0C {
                self.channels[channel].io + reg - 0x06
            } else if reg < 0x0E {
                self.channels[channel].control + reg - 0x0A
            } else if reg < 0x16 {
                self.channels[channel].bus_master + reg - 0x0E
            } else {
                return Err(error::Status::BadAddress);
            },
            value,
        );

        if reg > 0x07 && reg < 0x0C {
            outb(
                self.channels[channel].control + (REGISTER_CONTROL as u16) - 0x0A,
                self.channels[channel].n_ien,
            );
        }

        Ok(())
    }

    fn ioctrl(&mut self, code: usize, argument: usize) -> error::Result<usize> {
        match code {
            IOCTRL_ENUMERATE => self.enumerate_drives(),
            IOCTRL_POLL => self.polling((argument & 1) as u8, false),
            IOCTRL_ADVANCED_POLL => self.polling((argument & 1) as u8, true),
            IOCTRL_SET_CHANNEL_INTERRUPT => {
                if argument >= 2 {
                    Err(error::Status::InvalidRequestCode)
                } else {
                    self.channels[argument].n_ien = 0;
                    Ok(0)
                }
            }
            IOCTRL_CLEAR_CHANNEL_INTERRUPT => {
                if argument >= 2 {
                    Err(error::Status::InvalidRequestCode)
                } else {
                    self.channels[argument].n_ien = 2;
                    Ok(0)
                }
            }
            _ => Err(error::Status::InvalidIOCtrl),
        }
    }
}
