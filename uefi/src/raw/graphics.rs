use base::PhysicalAddress;

#[repr(C)]
pub struct GraphicsMode {
    horizontal_resolution: u32,
    vertical_resolution: u32,
    pixel_format: u32,
    red_mask: u32,
    green_mask: u32,
    blue_mask: u32,
    pixels_per_scanline: u32,
    framebuffer: PhysicalAddress,
    framebuffer_size: usize,
}

impl GraphicsMode {
    pub(crate) fn horizontal_resolution(&self) -> u32 {
        self.horizontal_resolution
    }

    pub(crate) fn vertical_resolution(&self) -> u32 {
        self.vertical_resolution
    }

    pub(crate) fn pixels_per_scanline(&self) -> u32 {
        self.pixels_per_scanline
    }

    pub(crate) fn framebuffer(&self) -> PhysicalAddress {
        self.framebuffer
    }

    pub(crate) fn framebuffer_size(&self) -> usize {
        self.framebuffer_size
    }
}
