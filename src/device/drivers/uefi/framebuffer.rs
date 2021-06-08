use crate::{bootloader, session::color::Color};
use core::{convert::TryInto, ptr::null_mut};

pub struct Framebuffer {
    width: u32,
    height: u32,
    pixels_per_scanline: u32,
    buffer: *mut Color,
    size: usize,
}

impl Framebuffer {
    pub fn new(gmode: *const bootloader::GraphicsMode) -> Self {
        if gmode.is_null() {
            return Framebuffer::null();
        }

        let framebuffer = unsafe {
            if ((*gmode).framebuffer as usize) < crate::memory::KERNEL_VMA {
                (*gmode).framebuffer as usize + crate::memory::KERNEL_VMA
            } else {
                (*gmode).framebuffer as usize
            }
        } as *mut Color;

        unsafe {
            Framebuffer {
                width: (*gmode).horizontal_resolution,
                height: (*gmode).vertical_resolution,
                pixels_per_scanline: (*gmode).pixels_per_scanline,
                buffer: framebuffer,
                size: (*gmode).framebuffer_size,
            }
        }
    }

    pub const fn null() -> Self {
        Framebuffer {
            width: 0,
            height: 0,
            pixels_per_scanline: 0,
            buffer: null_mut(),
            size: 0,
        }
    }

    pub fn put_pixel(&self, x: u32, y: u32, color: Color) {
        if x >= self.width || y >= self.height || self.buffer.is_null() {
            return;
        }

        unsafe {
            *(self
                .buffer
                .offset((x + y * self.pixels_per_scanline) as isize)) = color;
        }
    }

    pub fn clear(&self, color: Color) {
        if self.buffer.is_null() {
            return;
        }

        let mut i: isize = 0;
        while i < self.size.try_into().unwrap() {
            unsafe {
                *(self.buffer.offset(i)) = color;
            }

            i += 1;
        }
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }
}

unsafe impl Send for Framebuffer {}
unsafe impl Sync for Framebuffer {}
