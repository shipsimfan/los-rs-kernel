use crate::aml::{impl_core_display, pkg_length, term_objects::TermArg, Display, Result, Stream};
use alloc::{boxed::Box, vec::Vec};

pub(in crate::aml::term_objects) struct Buffer {
    buffer_size: Box<TermArg>,
    bytes: Vec<u8>,
}

impl Buffer {
    pub(super) fn parse(stream: &mut Stream) -> Result<Self> {
        let mut stream = pkg_length::parse_to_stream(stream)?;

        let buffer_size = Box::new(TermArg::parse(&mut stream)?);
        let bytes = stream.collect();

        Ok(Buffer { buffer_size, bytes })
    }
}

impl Display for Buffer {
    fn display(&self, f: &mut core::fmt::Formatter, depth: usize, _: bool) -> core::fmt::Result {
        self.display_prefix(f, depth)?;
        write!(f, "Buffer ({}) {{", self.buffer_size)?;

        for i in 0..self.bytes.len() {
            write!(f, " {:#02X}", self.bytes[i])?;

            if i < self.bytes.len() - 1 {
                write!(f, ",")?;
            } else {
                write!(f, " ")?;
            }
        }

        write!(f, "}}")
    }
}

impl_core_display!(Buffer);
