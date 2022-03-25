use crate::{
    bootloader,
    device::{Device, DeviceReference},
    error,
    session::console::{self, Color},
};
use alloc::{boxed::Box, str};
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
    cursor_state: bool,
}

pub fn initialize(gmode: *const bootloader::GraphicsMode) {
    let mut framebuffer = Framebuffer::new(gmode);
    framebuffer.clear(0);

    let console = DeviceReference::new(Box::new(UEFIConsole {
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
        cursor_state: false,
    }));

    crate::device::register_device("/boot_video", console)
        .expect("Failed to register UEFI boot video console!");
    crate::logger::enable_boot_video_logging();
}

impl UEFIConsole {
    fn set_cursor_state(&mut self, state: bool) {
        if self.cursor_state && !state {
            self.clear_cursor();
        }

        self.cursor_state = state;
        self.render_cursor();
    }

    fn render_cursor(&mut self) {
        if self.cursor_state {
            self.render_character('_', true);
        }
    }

    fn clear_cursor(&mut self) {
        if self.cursor_state {
            self.render_character(' ', true);
        }
    }

    fn render_character(&mut self, c: char, ignore_style: bool) {
        const MASK: [u8; 8] = [128, 64, 32, 16, 8, 4, 2, 1];
        let glyph = (c as u32) * 16;

        let bx = self.cx * 8;
        let by = self.cy * 16;

        for cy in 0..16 {
            for cx in 0..8 {
                let color = if font::FONT[(glyph + cy) as usize] & MASK[cx as usize] == 0 {
                    if !ignore_style && self.strikethrough && cy == 8 && c != ' ' {
                        &self.foreground
                    } else if !ignore_style && self.underline && cy == 15 && c != ' ' {
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
        self.render_cursor()
    }

    pub fn print(&mut self, string: &str) {
        self.clear_cursor();

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
                        self.render_character(' ', false);
                    }
                }
                '\t' => {
                    if self.cx >= self.width - 4 {
                        self.cx = 0;
                        self.cy += 1;
                    } else {
                        if self.cx % 4 == 0 {
                            self.render_character(' ', false);
                            self.cx += 1;
                        }

                        while self.cx % 4 != 0 {
                            self.render_character(' ', false);
                            self.cx += 1;
                        }
                    }
                }
                _ => {
                    self.render_character(c, false);
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

        self.render_cursor()
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
                    self.clear_cursor();
                    self.cx = argument;
                    self.render_cursor();
                    Ok(argument as usize)
                } else {
                    Err(error::Status::OutOfRange)
                }
            }
            console::IOCTRL_SET_CURSOR_Y => {
                let argument = (argument & 0xFFFFFFFF) as u32;
                if argument < self.height {
                    self.clear_cursor();
                    self.cy = argument;
                    self.render_cursor();
                    Ok(argument as usize)
                } else {
                    Err(error::Status::OutOfRange)
                }
            }
            console::IOCTRL_GET_WIDTH => Ok(self.width as usize),
            console::IOCTRL_GET_HEIGHT => Ok(self.height as usize),
            console::IOCTRL_SET_CURSOR_STATE => {
                self.set_cursor_state(argument != 0);
                Ok(0)
            }
            _ => Err(error::Status::InvalidIOCtrl),
        }
    }
}
