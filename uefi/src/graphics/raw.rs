#[repr(C)]
pub struct GraphicsMode {
    pub horizontal_resolution: u32,
    pub vertical_resolution: u32,
    pub pixel_format: u32,
    pub red_mask: u32,
    pub green_mask: u32,
    pub blue_mask: u32,
    pub pixels_per_scanline: u32,
    pub framebuffer: *mut u32,
    pub framebuffer_size: usize,
}
