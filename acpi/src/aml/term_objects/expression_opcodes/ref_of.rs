use crate::aml::{impl_core_display, super_name::SuperName, Display, Result, Stream};
use alloc::boxed::Box;

pub(in crate::aml) struct RefOf {
    object: Box<SuperName>,
}

impl RefOf {
    pub(super) fn parse(stream: &mut Stream) -> Result<Self> {
        let object = Box::new(SuperName::parse(stream)?);

        Ok(RefOf { object })
    }
}

impl Display for RefOf {
    fn display(&self, f: &mut core::fmt::Formatter, depth: usize, _: bool) -> core::fmt::Result {
        self.display_prefix(f, depth)?;
        write!(f, "Ref Of ({})", self.object)
    }
}

impl_core_display!(RefOf);
