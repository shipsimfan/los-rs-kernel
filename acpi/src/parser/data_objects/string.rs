use crate::parser::{Result, Stream};
use alloc::{borrow::ToOwned, vec::Vec};

#[derive(Clone)]
pub(crate) struct String<'a> {
    inner: &'a [u8],
}

impl<'a> String<'a> {
    pub(super) fn parse(stream: &mut Stream<'a>) -> Result<Self> {
        let inner = stream.collect_until(0x00)?;

        Ok(String { inner })
    }

    pub(crate) fn to_vec(&self) -> Vec<u8> {
        self.inner.to_owned()
    }
}

impl<'a> core::fmt::Display for String<'a> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "\"")?;
        for byte in self.inner {
            write!(f, "{}", *byte as char)?;
        }
        write!(f, "\"")
    }
}
