use alloc::vec::Vec;

pub(crate) struct String(Vec<u8>);

impl String {
    pub(crate) unsafe fn new_unchecked(string: Vec<u8>) -> Self {
        String(string)
    }
}

impl core::fmt::Display for String {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "\"")?;

        for byte in &self.0 {
            write!(f, "{}", *byte as char)?;
        }

        write!(f, "\"")
    }
}
