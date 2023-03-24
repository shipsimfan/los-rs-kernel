use crate::aml::{impl_core_display, next, Display, NameString, Result, Stream};

pub(in crate::aml::term_objects) struct External {
    offset: usize,
    name: NameString,
    object_type: u8,
    argument_count: u8,
}

impl External {
    pub(super) fn parse(stream: &mut Stream) -> Result<Self> {
        let offset = stream.offset() - 2;

        let name = NameString::parse(stream)?;
        let object_type = next!(stream);
        let argument_count = next!(stream);

        Ok(External {
            offset,
            name,
            object_type,
            argument_count,
        })
    }
}

impl Display for External {
    fn display(&self, f: &mut core::fmt::Formatter, depth: usize) -> core::fmt::Result {
        self.display_prefix(f, depth)?;
        writeln!(
            f,
            "External {} ({} - {}) @ {}",
            self.name, self.object_type, self.argument_count, self.offset
        )
    }
}

impl_core_display!(External);
