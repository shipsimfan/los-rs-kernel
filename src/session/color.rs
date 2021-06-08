#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct Color(pub u32);

impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Color((b as u32) | ((g as u32) << 8) | ((r as u32) << 16))
    }
}
