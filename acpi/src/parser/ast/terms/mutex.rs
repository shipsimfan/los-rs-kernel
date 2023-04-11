use crate::{
    display_prefix, impl_core_display,
    parser::{name_string, next, Result, Stream},
    Display, Path,
};

pub(crate) struct Mutex {
    path: Path,
    sync_level: u8,
}

impl Mutex {
    pub(super) fn parse(stream: &mut Stream) -> Result<Self> {
        let path = name_string::parse(stream, "Mutex")?;
        let sync_level = next!(stream, "Mutex") & 0x0F;

        Ok(Mutex { path, sync_level })
    }
}

impl Display for Mutex {
    fn display(&self, f: &mut core::fmt::Formatter, depth: usize, _: bool) -> core::fmt::Result {
        display_prefix!(f, depth);
        writeln!(f, "Mutex ({}, {})", self.path, self.sync_level)
    }
}

impl_core_display!(Mutex);
