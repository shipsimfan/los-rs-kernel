pub trait Device: Send {
    fn read(&self, address: usize, buffer: &mut [u8]) -> base::error::Result<usize>;
    fn write(&mut self, address: usize, buffer: &[u8]) -> base::error::Result<usize>;

    fn read_register(&mut self, address: usize) -> base::error::Result<usize>;
    fn write_register(&mut self, address: usize, value: usize) -> base::error::Result<()>;

    fn ioctrl(&mut self, code: usize, argument: usize) -> base::error::Result<usize>;
}
