use crate::aml::{impl_core_display, next, Display, NameString, Result, Stream};

pub(in crate::aml::term_objects) struct Mutex {
    offset: usize,
    name: NameString,

    sync_level: u8,
}

impl Mutex {
    pub(super) fn parse(stream: &mut Stream) -> Result<Self> {
        let offset = stream.offset() - 2;

        let name = NameString::parse(stream)?;

        let sync_level = next!(stream) & 0x0F;

        Ok(Mutex {
            offset,
            name,
            sync_level,
        })
    }
}

impl Display for Mutex {
    fn display(&self, f: &mut core::fmt::Formatter, depth: usize) -> core::fmt::Result {
        self.display_prefix(f, depth)?;
        writeln!(
            f,
            "Mutex {} ({}) @ {}",
            self.name, self.sync_level, self.offset
        )
    }
}

impl_core_display!(Mutex);
