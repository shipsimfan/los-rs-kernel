use crate::aml::{impl_core_display, next, Display, NameString, Result, Stream};

pub(in crate::aml::term_objects) struct External {
    name: NameString,
    object_type: u8,
    argument_count: u8,
}

impl External {
    pub(super) fn parse(stream: &mut Stream) -> Result<Self> {
        let name = NameString::parse(stream)?;
        let object_type = next!(stream);
        let argument_count = next!(stream);

        Ok(External {
            name,
            object_type,
            argument_count,
        })
    }
}

impl Display for External {
    fn display(&self, f: &mut core::fmt::Formatter, depth: usize, _: bool) -> core::fmt::Result {
        self.display_prefix(f, depth)?;
        writeln!(
            f,
            "External ({}, {}, {})",
            self.name, self.object_type, self.argument_count
        )
    }
}

impl_core_display!(External);
