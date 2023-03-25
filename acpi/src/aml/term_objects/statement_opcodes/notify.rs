use crate::aml::{
    impl_core_display, super_name::SuperName, term_objects::TermArg, Display, Result, Stream,
};

pub(in crate::aml::term_objects) struct Notify {
    name: SuperName,
    value: TermArg,
}

impl Notify {
    pub(super) fn parse(stream: &mut Stream) -> Result<Self> {
        let name = SuperName::parse(stream)?;
        let value = TermArg::parse(stream)?;

        Ok(Notify { name, value })
    }
}

impl Display for Notify {
    fn display(&self, f: &mut core::fmt::Formatter, depth: usize, _: bool) -> core::fmt::Result {
        self.display_prefix(f, depth)?;
        write!(f, "Notify ({}, {})", self.name, self.value)
    }
}

impl_core_display!(Notify);
