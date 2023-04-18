use crate::{
    display_prefix, impl_core_display,
    parser::{name_string, Result, Stream},
    Display, Path,
};

pub(crate) struct Alias {
    path: Path,
    target: Path,
}

impl Alias {
    pub(super) fn parse(stream: &mut Stream) -> Result<Self> {
        let path = name_string::parse(stream, "Alias")?;
        let target = name_string::parse(stream, "Alias")?;

        Ok(Alias { path, target })
    }
}

impl Display for Alias {
    fn display(
        &self,
        f: &mut core::fmt::Formatter,
        depth: usize,
        _: bool,
        newline: bool,
    ) -> core::fmt::Result {
        display_prefix!(f, depth);
        write!(f, "Alias ({}, {})", self.path, self.target)?;

        if newline {
            writeln!(f)
        } else {
            Ok(())
        }
    }
}

impl_core_display!(Alias);
