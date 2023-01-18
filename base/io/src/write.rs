use error::{Error, ErrorKind};

pub trait Write {
    fn write(&mut self, buffer: &[u8]) -> Result<usize, Error>;

    fn write_all(&mut self, mut buffer: &[u8]) -> Result<(), Error> {
        while buffer.len() > 0 {
            match self.write(buffer) {
                Ok(change) => buffer = &buffer[change..],
                Err(error) => match error.kind() {
                    ErrorKind::Interrupted => continue,
                    _ => return Err(error),
                },
            }
        }

        Ok(())
    }
}
