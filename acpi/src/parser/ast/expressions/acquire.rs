use crate::parser::{next, Context, Result, Stream, SuperName};

pub(crate) struct Acquire<'a> {
    mutex: SuperName<'a>,
    timeout: u16,
}

impl<'a> Acquire<'a> {
    pub(super) fn parse(stream: &mut Stream<'a>, context: &mut Context) -> Result<Self> {
        let mutex = SuperName::parse(stream, context)?;
        let timeout = u16::from_le_bytes([next!(stream, "Acquire"), next!(stream, "Acquire")]);

        Ok(Acquire { mutex, timeout })
    }
}

impl<'a> core::fmt::Display for Acquire<'a> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Acquire ({}, {})", self.mutex, self.timeout)
    }
}
