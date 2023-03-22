use crate::aml::{impl_core_display, next, pkg_length, Display, Result, Stream};

pub(in crate::aml::term_objects) struct Package {
    offset: usize,
}

impl Package {
    pub(super) fn parse(stream: &mut Stream) -> Result<Self> {
        let offset = stream.offset() - 1;

        let mut stream = pkg_length::parse_to_stream(stream)?;

        Ok(Package { offset })
    }
}

impl Display for Package {
    fn display(&self, f: &mut core::fmt::Formatter, depth: usize) -> core::fmt::Result {
        self.display_prefix(f, depth)?;
        writeln!(f, "Package @ {}", self.offset)
    }
}

impl_core_display!(Package);
