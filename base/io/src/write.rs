use error::Error;

pub trait Write {
    fn write(&mut self, buffer: &[u8]) -> Result<usize, Error>;

    fn write_all(&mut self, mut buffer: &[u8]) -> Result<(), Error> {
        while buffer.len() > 0 {
            let change = self.write(buffer)?;
            buffer = &buffer[change..];
        }

        Ok(())
    }
}
