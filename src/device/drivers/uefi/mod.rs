use crate::{
    bootloader,
    device::{Device, DeviceBox},
    error,
    locks::Mutex,
    session::{color::Color, CONSOLE_IOCTRL_CLEAR},
};
use alloc::{boxed::Box, str, sync::Arc};
use core::fmt;
use framebuffer::Framebuffer;

mod font;
mod framebuffer;

struct UEFIConsole {
    framebuffer: Framebuffer,
    width: u32,
    height: u32,
    cx: u32,
    cy: u32,
    foreground: Color,
    background: Color,
}

struct DeviceLogger {
    output: DeviceBox,
}

pub fn initialize(gmode: *const bootloader::GraphicsMode) {
    let framebuffer = Framebuffer::new(gmode);
    framebuffer.clear(Color(0));

    let console: DeviceBox = Arc::new(Mutex::new(Box::new(UEFIConsole {
        width: framebuffer.width() / 8,
        height: framebuffer.height() / 16,
        framebuffer: framebuffer,
        cx: 0,
        cy: 0,
        foreground: Color::new(0xFF, 0xFF, 0xFF),
        background: Color(0),
    })));

    let logger = Box::new(DeviceLogger {
        output: console.clone(),
    });

    crate::device::register_device("/uefi_console", console)
        .expect("Failed to register UEFI console!");
    crate::logger::set_logger(logger);
}

impl UEFIConsole {
    fn render_character(&self, c: char) {
        const MASK: [u8; 8] = [128, 64, 32, 16, 8, 4, 2, 1];
        let glyph = (c as u32) * 16;

        let bx = self.cx * 8;
        let by = self.cy * 16;

        let mut cy = 0;
        while cy < 16 {
            let mut cx = 0;
            while cx < 8 {
                let color = if font::FONT[(glyph + cy) as usize] & MASK[cx as usize] == 0 {
                    self.background
                } else {
                    self.foreground
                };

                self.framebuffer.put_pixel(bx + cx, by + cy, color);

                cx += 1;
            }

            cy += 1;
        }
    }

    fn clear_screen(&mut self) {
        self.framebuffer.clear(self.background);
    }

    pub fn print(&mut self, string: &str) {
        let mut iter = string.chars();
        while let Some(c) = iter.next() {
            match c {
                '\n' => {
                    self.cx = 0;
                    self.cy += 1;
                }
                '\r' => self.cx = 0,
                '\x08' => {
                    // backspace
                    if self.cx > 0 {
                        self.cx -= 1;
                        self.render_character(' ');
                    }
                }
                '\t' => {
                    if self.cx >= self.width - 4 {
                        self.cx = 0;
                        self.cy += 1;
                    } else {
                        if self.cx % 4 == 0 {
                            self.render_character(' ');
                            self.cx += 1;
                        }

                        while self.cx % 4 != 0 {
                            self.render_character(' ');
                            self.cx += 1;
                        }
                    }
                }
                '\x1B' => {
                    // Control character
                    let mut i = 0;
                    let mut chars: [char; 6] = ['\0', '\0', '\0', '\0', '\0', '\0'];
                    while let Some(c) = iter.next() {
                        if c == ']' {
                            match i {
                                0 => {
                                    self.set_color(Color::new(0xFF, 0xFF, 0xFF));
                                }
                                3 => {
                                    let mut j = 0;
                                    let mut color: [u8; 3] = [0, 0, 0];
                                    while j < 3 {
                                        if chars[j] >= 'a' && chars[j] <= 'f' {
                                            color[j] = chars[j] as u8 - 'a' as u8 + 10;
                                        } else if chars[j] >= 'A' && chars[j] <= 'F' {
                                            color[j] = chars[j] as u8 - 'A' as u8 + 10;
                                        } else {
                                            color[j] = chars[j] as u8 - '0' as u8;
                                        }

                                        color[j] *= 0x10;

                                        j += 1;
                                    }

                                    self.set_color(Color::new(color[0], color[1], color[2]));
                                }
                                6 => {
                                    let mut j = 0;
                                    let mut color: [u8; 3] = [0, 0, 0];
                                    while j < 3 {
                                        if chars[j * 2] >= 'a' && chars[j * 2] <= 'f' {
                                            color[j] = chars[j * 2] as u8 - 'a' as u8 + 10;
                                        } else if chars[j * 2] >= 'A' && chars[j * 2] <= 'F' {
                                            color[j] = chars[j * 2] as u8 - 'A' as u8 + 10;
                                        } else {
                                            color[j] = chars[j * 2] as u8 - '0' as u8;
                                        }

                                        color[j] *= 0x10;

                                        if chars[j * 2 + 1] >= 'a' && chars[j * 2 + 1] <= 'f' {
                                            color[j] += chars[j * 2 + 1] as u8 - 'a' as u8 + 10;
                                        } else if chars[j * 2 + 1] >= 'A' && chars[j * 2 + 1] <= 'F'
                                        {
                                            color[j] += chars[j * 2 + 1] as u8 - 'A' as u8 + 10;
                                        } else {
                                            color[j] += chars[j * 2 + 1] as u8 - '0' as u8;
                                        }

                                        j += 1;
                                    }

                                    self.set_color(Color::new(color[0], color[1], color[2]));
                                }
                                _ => break,
                            }

                            break;
                        }

                        if i > 6
                            || !((c >= 'a' && c <= 'f')
                                || (c >= 'A' || c <= 'F')
                                || (c >= '0' || c <= '9'))
                        {
                            break;
                        }

                        chars[i] = c;
                        i += 1;
                    }
                }
                _ => {
                    self.render_character(c);
                    self.cx += 1;
                }
            }

            if self.cx >= self.width {
                self.cy += 1;
                self.cx = 0;
            }

            if self.cy >= self.height {
                self.framebuffer.scroll_up(16);
                self.cy -= 1;
            }
        }
    }

    pub fn set_color(&mut self, color: Color) {
        self.foreground = color;
    }
}

impl Device for UEFIConsole {
    fn write(&mut self, _address: usize, buffer: &[u8]) -> error::Result {
        self.print(match str::from_utf8(buffer) {
            Err(_) => return Err(error::Status::InvalidArgument),
            Ok(str) => str,
        });

        Ok(())
    }

    fn read(&self, _: usize, _: &mut [u8]) -> error::Result {
        Err(error::Status::NotSupported)
    }

    fn read_register(&mut self, _: usize) -> Result<usize, error::Status> {
        Err(error::Status::NotSupported)
    }

    fn write_register(&mut self, _: usize, _: usize) -> error::Result {
        Err(error::Status::NotSupported)
    }

    fn ioctrl(&mut self, code: usize, _: usize) -> Result<usize, error::Status> {
        match code {
            CONSOLE_IOCTRL_CLEAR => {
                self.cx = 0;
                self.cy = 0;
                self.clear_screen();
                Ok(0)
            }
            _ => Err(error::Status::NotSupported),
        }
    }
}

impl fmt::Write for DeviceLogger {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        match self.output.lock().write(0, s.as_bytes()) {
            Err(_) => Err(fmt::Error {}),
            Ok(()) => Ok(()),
        }
    }
}
