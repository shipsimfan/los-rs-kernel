use crate::{
    display_prefix, impl_core_display,
    parser::{name_string, Result, Stream},
    Display, Path,
};

pub(crate) struct Event {
    path: Path,
}

impl Event {
    pub(super) fn parse(stream: &mut Stream) -> Result<Self> {
        let path = name_string::parse(stream, "Event")?;

        Ok(Event { path })
    }
}

impl Display for Event {
    fn display(
        &self,
        f: &mut core::fmt::Formatter,
        depth: usize,
        _: bool,
        newline: bool,
    ) -> core::fmt::Result {
        display_prefix!(f, depth);
        write!(f, "Event ({})", self.path)?;

        if newline {
            writeln!(f)
        } else {
            Ok(())
        }
    }
}

impl_core_display!(Event);
