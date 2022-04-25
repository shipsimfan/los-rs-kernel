use alloc::boxed::Box;

pub fn handler(
    _: usize,
    _: usize,
    _: usize,
    _: usize,
    _: usize,
    _: usize,
    _: interrupts::Registers,
    _: u64,
    _: u64,
    _: u64,
) -> Result<isize, Box<dyn base::error::Error>> {
    Ok(0)
}
