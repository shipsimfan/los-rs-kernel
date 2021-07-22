use alloc::vec::Vec;

use crate::{bootloader, session::color::Color};

pub struct Framebuffer {
    width: u32,
    height: u32,
    pixels_per_scanline: u32,
    front_buffer: &'static mut [Color],
    back_buffer: Vec<Color>,
}

impl Framebuffer {
    pub fn new(gmode: *const bootloader::GraphicsMode) -> Self {
        let framebuffer = unsafe {(*gmode).framebuffer} as usize;
        let framebuffer_size_bytes = unsafe {(*gmode).framebuffer_size};
        let framebuffer_size = framebuffer_size_bytes / core::mem::size_of::<Color>();

        let framebuffer = if framebuffer < crate::memory::KERNEL_VMA {
                framebuffer + crate::memory::KERNEL_VMA
        } else {
                framebuffer
        } as *mut Color; 
        let framebuffer = unsafe {core::slice::from_raw_parts_mut(framebuffer, framebuffer_size)};

        // Create back buffer
        let mut back_buffer = Vec::with_capacity(framebuffer_size);

        // Copy front buffer
        for i in 0..framebuffer_size {
            back_buffer.push(framebuffer[i]);
        }

        unsafe {
            Framebuffer {
                width: (*gmode).horizontal_resolution,
                height: (*gmode).vertical_resolution,
                pixels_per_scanline: (*gmode).pixels_per_scanline,
                front_buffer: framebuffer,
                back_buffer: back_buffer,
            }
        }
    }

    pub fn put_pixel(&mut self, x: u32, y: u32, color: Color) {
        if x >= self.width || y >= self.height
        {
            return;
        }

        let idx = (x + y * self.pixels_per_scanline) as usize;
    
        self.front_buffer[idx] = color;
        self.back_buffer[idx] = color;
    }

    pub fn clear(&mut self, color: Color) {
        let mut i: usize = 0;
        for value in &mut self.back_buffer {
            *value = color;
            self.front_buffer[i] = color;

            i += 1;
        }
    }

    pub fn scroll_up(&mut self, amount: usize) {
        let amount = amount / 2;

        let front_buffer = self.front_buffer.as_mut_ptr() as *mut usize;
        let back_buffer = self.back_buffer.as_mut_ptr() as *mut usize;
        let diff = amount * self.pixels_per_scanline as usize;
        let buffer_size = self.front_buffer.len() / 2;

        let mut i = 0isize;
        while i < (buffer_size - diff) as isize {
            unsafe {
                let val = *back_buffer.offset(i + diff as isize);
                *front_buffer.offset(i) = val;
                *back_buffer.offset(i) = val;
            }

            i += 1;
        }

        while i < buffer_size as isize {
            unsafe {
                *front_buffer.offset(i) = 0;
                *back_buffer.offset(i) = 0;
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