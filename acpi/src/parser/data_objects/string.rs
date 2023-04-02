use crate::parser::{next, Result, Stream};
use alloc::vec::Vec;

#[derive(Clone)]
pub(crate) struct String {
    inner: Vec<u8>,
}

impl String {
    pub(super) fn parse(stream: &mut Stream) -> Result<Self> {
        let mut inner = Vec::new();
        loop {
            let c = next!(stream);

            if c == 0x00 {
                return Ok(String { inner });
            }

            inner.push(c);
        }
    }
}

impl core::fmt::Display for String {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        for byte in &self.inner {
            write!(f, "{}", *byte as char)?;
        }

        Ok(())
    }
}
