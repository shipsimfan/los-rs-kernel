use crate::aml::{impl_core_display, next, super_name::SuperName, Display, Result, Stream};

pub(in crate::aml::term_objects) struct Acquire {
    mutex: SuperName,
    timeout: u16,
}

impl Acquire {
    pub(super) fn parse(stream: &mut Stream) -> Result<Self> {
        let mutex = SuperName::parse(stream)?;
        let timeout = u16::from_le_bytes([next!(stream), next!(stream)]);

        Ok(Acquire { mutex, timeout })
    }
}

impl Display for Acquire {
    fn display(&self, f: &mut core::fmt::Formatter, depth: usize, _: bool) -> core::fmt::Result {
        self.display_prefix(f, depth)?;
        write!(f, "Acquire ({}, {})", self.mutex, self.timeout)
    }
}

impl_core_display!(Acquire);
