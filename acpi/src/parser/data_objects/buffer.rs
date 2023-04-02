use crate::parser::{pkg_length, Argument, Result, Stream};
use alloc::{borrow::ToOwned, boxed::Box, vec::Vec};

pub(crate) struct Buffer<'a> {
    buffer_size: Box<Argument<'a>>,
    buffer: &'a [u8],
}

impl<'a> Buffer<'a> {
    pub(super) fn parse(stream: &mut Stream<'a>) -> Result<Self> {
        let mut stream = pkg_length::parse_to_stream(stream)?;

        let buffer_size = Box::new(Argument::parse(&mut stream)?);
        let buffer = stream.collect_remaining()?;

        Ok(Buffer {
            buffer_size,
            buffer,
        })
    }

    pub(crate) fn buffer_size(&self) -> &Argument {
        &self.buffer_size
    }

    pub(crate) fn to_vec(&self, buffer_size: u64) -> Vec<u8> {
        self.buffer[..buffer_size as usize].to_owned()
    }
}

impl<'a> core::fmt::Display for Buffer<'a> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Buffer ({})", self.buffer_size)
    }
}
