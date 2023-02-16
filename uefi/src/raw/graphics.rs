#[repr(C)]
pub struct GraphicsMode {
    horizontal_resolution: u32,
    vertical_resolution: u32,
    pixel_format: u32,
    red_mask: u32,
    green_mask: u32,
    blue_mask: u32,
    pixels_per_scanline: u32,
    framebuffer: *mut u32,
    framebuffer_size: usize,
}
