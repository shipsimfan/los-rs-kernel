use core::fmt::Write;

pub trait BootVideo: Write + 'static {
    fn framebuffer_memory(&self) -> (usize, usize);
}
