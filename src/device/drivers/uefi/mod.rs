use crate::{
    bootloader,
    device::{Device, DeviceBox},
    error,
    locks::Mutex,
    session::console::{self, Color},
};
use alloc::{boxed::Box, str, sync::Arc};
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
    dim: bool,
    strikethrough: bool,
    underline: bool,
}

pub fn initialize(gmode: *const bootloader::GraphicsMode) {
    let mut framebuffer = Framebuffer::new(gmode);
    framebuffer.clear(0);

    let console: DeviceBox = Arc::new(Mutex::new(Box::new(UEFIConsole {
        width: framebuffer.width() / 8,
        height: framebuffer.height() / 16,
        framebuffer: framebuffer,
        cx: 0,
        cy: 0,
        foreground: Color::new(0xFF, 0xFF, 0xFF),
        background: Color::new(0x00, 0x00, 0x00),
        dim: false,
        strikethrough: false,
        underline: false,
    })));

    crate::device::register_device("/boot_video", console)
        .expect("Failed to register UEFI boot video console!");
    crate::logger::enable_boot_video_logging();
}

impl UEFIConsole {
    fn render_character(&mut self, c: char) {
        const MASK: [u8; 8] = [128, 64, 32, 16, 8, 4, 2, 1];
        let glyph = (c as u32) * 16;

        let bx = self.cx * 8;
        let by = self.cy * 16;

        for cy in 0..16 {
            for cx in 0..8 {
                let color = if font::FONT[(glyph + cy) as usize] & MASK[cx as usize] == 0 {
                    if self.strikethrough && cy == 8 && c != ' ' {
                        &self.foreground
                    } else if self.underline && cy == 15 && c != ' ' {
                        &self.foreground
                    } else {
                        &self.background
                    }
                } else {
                    &self.foreground
                };

                let color = if self.dim {
                    Color::average(color, &self.background)
                } else {
                    color.clone()
                };

                self.framebuffer
                    .put_pixel(bx + cx, by + cy, color.as_usize() as u32);
            }
        }
    }

    fn clear_screen(&mut self) {
        self.framebuffer.clear(self.background.as_usize() as u32);
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
}

impl Device for UEFIConsole {
    fn write(&mut self, _address: usize, buffer: &[u8]) -> error::Result<()> {
        self.print(match str::from_utf8(buffer) {
            Err(_) => return Err(error::Status::InvalidUTF8),
            Ok(str) => str,
        });

        Ok(())
    }

    fn read(&self, _: usize, _: &mut [u8]) -> error::Result<()> {
        Err(error::Status::NotSupported)
    }

    fn read_register(&mut self, _: usize) -> error::Result<usize> {
        Err(error::Status::NotSupported)
    }

    fn write_register(&mut self, _: usize, _: usize) -> error::Result<()> {
        Err(error::Status::NotSupported)
    }

    fn ioctrl(&mut self, code: usize, argument: usize) -> error::Result<usize> {
        match code {
            console::IOCTRL_CLEAR => {
                self.cx = 0;
                self.cy = 0;
                self.clear_screen();
                Ok(0)
            }
            console::IOCTRL_SET_ATTRIBUTE => {
                self.dim = argument & console::STYLE_DIM != 0;
                self.strikethrough = argument & console::STYLE_STRIKETRHOUGH != 0;
                self.underline = argument & console::STYLE_UNDERLINE != 0;

                Ok(0)
            }
            console::IOCTRL_SET_FOREGROUND_COLOR => {
                self.foreground = Color::from_usize(argument);
                Ok(0)
            }
            console::IOCTRL_SET_BACKGROUND_COLOR => {
                self.background = Color::from_usize(argument);
                Ok(0)
            }
            console::IOCTRL_SET_CURSOR_X => {
                let argument = (argument & 0xFFFFFFFF) as u32;
                if argument < self.width {
                    self.cx = argument;
                    Ok(argument as usize)
                } else {
                    Err(error::Status::OutOfRange)
                }
            }
            console::IOCTRL_SET_CURSOR_Y => {
                let argument = (argument & 0xFFFFFFFF) as u32;
                if argument < self.height {
                    self.cy = argument;
                    Ok(argument as usize)
                } else {
                    Err(error::Status::OutOfRange)
                }
            }
            console::IOCTRL_GET_WIDTH => Ok(self.width as usize),
            console::IOCTRL_GET_HEIGHT => Ok(self.height as usize),
            _ => Err(error::Status::InvalidIOCtrl),
        }
    }
}
