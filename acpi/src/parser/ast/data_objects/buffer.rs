use crate::parser::{ast::Argument, pkg_length, Context, Result, Stream};
use alloc::boxed::Box;

pub(crate) struct Buffer<'a> {
    size: Box<Argument<'a>>,
    initial_data: &'a [u8],
}

impl<'a> Buffer<'a> {
    pub(super) fn parse(stream: &mut Stream<'a>, context: &mut Context) -> Result<Self> {
        let mut stream = pkg_length::parse_to_stream(stream, "Buffer")?;

        let size = Box::new(Argument::parse(&mut stream, context)?);
        let initial_data = stream.collect_remaining("Buffer")?;

        Ok(Buffer { size, initial_data })
    }
}

impl<'a> core::fmt::Display for Buffer<'a> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Buffer ({}, [", self.size)?;

        for i in 0..self.initial_data.len() {
            write!(f, "{:#04X}", self.initial_data[i])?;

            if i < self.initial_data.len() - 1 {
                write!(f, ", ")?;
            }
        }

        write!(f, "])")
    }
}
