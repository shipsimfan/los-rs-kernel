use crate::raw::GraphicsMode;
use core::ptr::NonNull;

pub(super) struct FrameBuffer {
    width: usize,
    height: usize,
    pixels_per_scanline: usize,
    front_buffer: NonNull<[u32]>,
}

impl FrameBuffer {
    pub(super) const fn null() -> Self {
        FrameBuffer {
            width: 0,
            height: 0,
            pixels_per_scanline: 0,
            front_buffer: unsafe {
                NonNull::slice_from_raw_parts(NonNull::new_unchecked(1 as *mut _), 1)
            },
        }
    }

    pub(super) fn new(graphics_mode: &GraphicsMode) -> Self {
        FrameBuffer {
            width: graphics_mode.horizontal_resolution() as usize,
            height: graphics_mode.vertical_resolution() as usize,
            pixels_per_scanline: graphics_mode.pixels_per_scanline() as usize,
            front_buffer: NonNull::slice_from_raw_parts(
                NonNull::new(graphics_mode.framebuffer().into_virtual()).unwrap(),
                graphics_mode.framebuffer_size(),
            ),
        }
    }

    pub(super) fn memory(&self) -> (usize, usize) {
        (
            self.front_buffer.as_ptr() as *const u32 as usize,
            self.front_buffer.len(),
        )
    }

    pub(super) fn width(&self) -> usize {
        self.width
    }

    pub(super) fn height(&self) -> usize {
        self.height
    }

    pub(super) fn put_pixel(&mut self, x: usize, y: usize, color: u32) {
        if x >= self.width || y >= self.height {
            return;
        }

        let idx = (x + y * self.pixels_per_scanline) as usize;

        unsafe { self.front_buffer.as_mut()[idx] = color };
    }

    pub(super) fn clear(&mut self, color: u32) {
        for value in unsafe { self.front_buffer.as_mut() } {
            *value = color;
        }
    }

    pub(super) fn scroll_up(&mut self, amount: usize) {
        let amount = amount / 2;

        let front_buffer = self.front_buffer.as_ptr() as *mut usize;
        let diff = amount * self.pixels_per_scanline as usize;
        let buffer_size = self.front_buffer.len() / 2;

        let mut i = 0isize;
        while i < (buffer_size - diff) as isize {
            unsafe {
                let val = *front_buffer.offset(i + diff as isize);
                *front_buffer.offset(i) = val;
            }

            i += 1;
        }

        while i < buffer_size as isize {
            unsafe { *front_buffer.offset(i) = 0 };

            i += 1;
        }
    }
}
