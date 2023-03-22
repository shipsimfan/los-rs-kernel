use crate::aml::{impl_core_display, Display, NameString, Result, Stream};

pub(in crate::aml::term_objects) struct Alias {
    offset: usize,
    name1: NameString,
    name2: NameString,
}

impl Alias {
    pub(super) fn parse(stream: &mut Stream) -> Result<Self> {
        let offset = stream.offset() - 1;

        let name1 = NameString::parse(stream)?;
        let name2 = NameString::parse(stream)?;

        Ok(Alias {
            offset,
            name1,
            name2,
        })
    }
}

impl Display for Alias {
    fn display(&self, f: &mut core::fmt::Formatter, depth: usize) -> core::fmt::Result {
        self.display_prefix(f, depth)?;
        writeln!(
            f,
            "Alias {} -> {} @ {}:",
            self.name1, self.name2, self.offset
        )
    }
}

impl_core_display!(Alias);
