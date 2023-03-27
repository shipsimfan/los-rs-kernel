use crate::aml::{
    impl_core_display, super_name::SuperName, term_objects::TermArg, Display, Result, Stream,
};
use alloc::boxed::Box;

pub(in crate::aml::term_objects) struct Store {
    arg: Box<TermArg>,
    name: SuperName,
}

impl Store {
    pub(super) fn parse(stream: &mut Stream) -> Result<Self> {
        let arg = Box::new(TermArg::parse(stream)?);
        let name = SuperName::parse(stream)?;

        Ok(Store { arg, name })
    }
}

impl Display for Store {
    fn display(&self, f: &mut core::fmt::Formatter, depth: usize, _: bool) -> core::fmt::Result {
        self.display_prefix(f, depth)?;
        write!(f, "Store ({}, {})", self.arg, self.name)
    }
}

impl_core_display!(Store);
