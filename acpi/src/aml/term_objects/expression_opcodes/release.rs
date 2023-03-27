use crate::aml::{impl_core_display, super_name::SuperName, Display, Result, Stream};

pub(in crate::aml::term_objects) struct Release {
    mutex: SuperName,
}

impl Release {
    pub(super) fn parse(stream: &mut Stream) -> Result<Self> {
        let mutex = SuperName::parse(stream)?;

        Ok(Release { mutex })
    }
}

impl Display for Release {
    fn display(&self, f: &mut core::fmt::Formatter, depth: usize, _: bool) -> core::fmt::Result {
        self.display_prefix(f, depth)?;
        write!(f, "Release ({})", self.mutex)
    }
}

impl_core_display!(Release);
