use crate::{bootloader, session::color::Color};

pub struct Framebuffer {
    width: u32,
    height: u32,
    pixels_per_scanline: u32,
    front_buffer: *mut Color,
    back_buffer: *mut Color,
    back_buffer_layout: alloc::alloc::Layout,
    size: usize,
}

impl Framebuffer {
    pub fn new(gmode: *const bootloader::GraphicsMode) -> Self {
        let framebuffer = unsafe {
            if ((*gmode).framebuffer as usize) < crate::memory::KERNEL_VMA {
                (*gmode).framebuffer as usize + crate::memory::KERNEL_VMA
            } else {
                (*gmode).framebuffer as usize
            }
        } as *mut Color;

        // Create back buffer
        let back_buffer_layout = unsafe {
            alloc::alloc::Layout::from_size_align_unchecked((*gmode).framebuffer_size, 8)
        };
        let back_buffer = unsafe { alloc::alloc::alloc(back_buffer_layout) } as *mut Color;

        // Copy front buffer
        let mut i: isize = 0;
        let top: isize =
            unsafe { (*gmode).framebuffer_size / core::mem::size_of::<Color>() } as isize;
        while i < top {
            unsafe { *back_buffer.offset(i) = *framebuffer.offset(i) };

            i += 1;
        }

        unsafe {
            Framebuffer {
                width: (*gmode).horizontal_resolution,
                height: (*gmode).vertical_resolution,
                pixels_per_scanline: (*gmode).pixels_per_scanline,
                front_buffer: framebuffer,
                back_buffer: back_buffer as *mut _,
                back_buffer_layout: back_buffer_layout,
                size: (*gmode).framebuffer_size,
            }
        }
    }

    pub fn put_pixel(&self, x: u32, y: u32, color: Color) {
        if x >= self.width
            || y >= self.height
            || self.front_buffer.is_null()
            || self.back_buffer.is_null()
        {
            return;
        }

        unsafe {
            *(self
                .front_buffer
                .offset((x + y * self.pixels_per_scanline) as isize)) = color;
            *(self
                .back_buffer
                .offset((x + y * self.pixels_per_scanline) as isize)) = color;
        }
    }

    pub fn clear(&self, color: Color) {
        if self.front_buffer.is_null() || self.back_buffer.is_null() {
            return;
        }

        let mut i: isize = 0;
        while i < (self.size / core::mem::size_of::<Color>()) as isize {
            unsafe {
                *(self.front_buffer.offset(i)) = color;
                *(self.back_buffer.offset(i)) = color;
            }

            i += 1;
        }
    }

    pub fn scroll_up(&self, amount: usize) {
        let amount = amount / 2;

        let front_buffer = self.front_buffer as *mut usize;
        let back_buffer = self.back_buffer as *mut usize;
        let diff = amount * self.pixels_per_scanline as usize;
        let buffer_size = self.size / core::mem::size_of::<usize>();

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

unsafe impl Send for Framebuffer {}
unsafe impl Sync for Framebuffer {}

impl Drop for Framebuffer {
    fn drop(&mut self) {
        unsafe { alloc::alloc::dealloc(self.back_buffer as *mut _, self.back_buffer_layout) };
    }
}
