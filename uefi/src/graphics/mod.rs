use crate::raw::GraphicsMode;
use base::BootVideo;
use core::{fmt::Write, ptr::NonNull};
use font::*;
use framebuffer::FrameBuffer;

mod font;
mod framebuffer;

pub struct UEFIBootVideo {
    framebuffer: FrameBuffer,

    width: usize,
    height: usize,
    cx: usize,
    cy: usize,
}

const FOREGROUND: u32 = u32::MAX;
const BACKGROUND: u32 = 0;

impl UEFIBootVideo {
    pub const fn null() -> Self {
        UEFIBootVideo {
            framebuffer: FrameBuffer::null(),

            width: 0,
            height: 0,
            cx: 0,
            cy: 0,
        }
    }

    pub fn initialize(&mut self, graphics_mode: NonNull<GraphicsMode>) {
        self.framebuffer = FrameBuffer::new(unsafe { graphics_mode.as_ref() });
        self.framebuffer.clear(0);

        self.width = self.framebuffer.width() / FONT_WIDTH;
        self.height = self.framebuffer.height() / FONT_HEIGHT;
    }

    fn render_character(&mut self, c: char) {
        const MASK: [u8; 8] = [128, 64, 32, 16, 8, 4, 2, 1];
        let glyph = (c as usize) * FONT_HEIGHT;

        let bx = self.cx * FONT_WIDTH;
        let by = self.cy * FONT_HEIGHT;

        for cy in 0..FONT_HEIGHT {
            for cx in 0..FONT_WIDTH {
                let color = if FONT[glyph + cy] & MASK[cx] == 0 {
                    BACKGROUND
                } else {
                    FOREGROUND
                };

                self.framebuffer.put_pixel(bx + cx, by + cy, color);
            }
        }
    }
}

impl BootVideo for UEFIBootVideo {
    fn framebuffer_memory(&self) -> (usize, usize) {
        self.framebuffer.memory()
    }
}

impl Write for UEFIBootVideo {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        let mut iter = s.chars();
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

        Ok(())
    }
}
