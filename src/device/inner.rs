pub trait Device: Send {
    fn read(&self, address: usize, buffer: &mut [u8]) -> crate::error::Result<()>;
    fn write(&mut self, address: usize, buffer: &[u8]) -> crate::error::Result<()>;

    fn read_register(&mut self, address: usize) -> crate::error::Result<usize>;
    fn write_register(&mut self, address: usize, value: usize) -> crate::error::Result<()>;

    fn ioctrl(&mut self, code: usize, argument: usize) -> crate::error::Result<usize>;
}
