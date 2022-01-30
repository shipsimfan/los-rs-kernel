pub trait File: Send {
    fn read(&mut self, offset: usize, buffer: &mut [u8]) -> crate::error::Result<isize>;
    fn write(&mut self, offset: usize, buffer: &[u8]) -> crate::error::Result<isize>;
    fn set_length(&mut self, new_length: usize) -> crate::error::Result<()>;
    fn get_length(&self) -> usize;
}
