use crate::aml::{impl_core_display, super_name::SuperName, Display, Result, Stream};

pub(in crate::aml::term_objects) struct Increment {
    name: SuperName,
}

impl Increment {
    pub(super) fn parse(stream: &mut Stream) -> Result<Self> {
        let name = SuperName::parse(stream)?;

        Ok(Increment { name })
    }
}

impl Display for Increment {
    fn display(&self, f: &mut core::fmt::Formatter, depth: usize, _: bool) -> core::fmt::Result {
        self.display_prefix(f, depth)?;
        write!(f, "Increment ({})", self.name)
    }
}

impl_core_display!(Increment);
