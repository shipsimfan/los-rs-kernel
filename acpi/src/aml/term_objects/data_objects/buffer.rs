use crate::aml::{impl_core_display, pkg_length, term_objects::TermArg, Display, Result, Stream};
use alloc::{boxed::Box, vec::Vec};

pub(in crate::aml::term_objects) struct Buffer {
    offset: usize,

    buffer_size: Box<TermArg>,
    bytes: Vec<u8>,
}

impl Buffer {
    pub(super) fn parse(stream: &mut Stream) -> Result<Self> {
        let offset = stream.offset() - 1;

        let mut stream = pkg_length::parse_to_stream(stream)?;

        let buffer_size = Box::new(TermArg::parse(&mut stream)?);
        let bytes = stream.collect();

        Ok(Buffer {
            offset,
            buffer_size,
            bytes,
        })
    }
}

impl Display for Buffer {
    fn display(&self, f: &mut core::fmt::Formatter, depth: usize) -> core::fmt::Result {
        self.display_prefix(f, depth)?;
        writeln!(f, "Buffer @ {}: {:?}", self.offset, self.bytes)?;
        self.buffer_size.display(f, depth + 1)
    }
}

impl_core_display!(Buffer);
