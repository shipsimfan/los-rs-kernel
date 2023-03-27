use crate::aml::{impl_core_display, next, Display, Error, Result, Stream};
use alloc::vec::Vec;

pub(in crate::aml::term_objects) struct String {
    string: Vec<u8>,
}

impl String {
    pub(super) fn parse(stream: &mut Stream) -> Result<Self> {
        let mut string = Vec::new();
        loop {
            let c = next!(stream);

            if c == 0x00 {
                break;
            } else if c > 0x7F {
                return Err(Error::unexpected_byte(c, stream.offset() - 1)).unwrap();
            }

            string.push(c);
        }

        Ok(String { string })
    }
}

impl Display for String {
    fn display(&self, f: &mut core::fmt::Formatter, depth: usize, _: bool) -> core::fmt::Result {
        self.display_prefix(f, depth)?;
        write!(f, "\"")?;

        for byte in &self.string {
            write!(f, "{}", *byte as char)?;
        }

        write!(f, "\"")
    }
}

impl_core_display!(String);
