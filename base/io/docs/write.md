# I/O - Write
This trait represents an I/O object which can be written to.

# Functions
The following functions are defined for the `Write` trait:

`write(&mut self, buffer: &[u8]) -> Result<usize, Error>`
: Writes the bytes from the buffer to the I/O object returning the number of bytes written

`write_all(&mut self, buffer: &[u8]) -> Result<(), Error>`
: Writes all the bytes from the buffer to the I/O object