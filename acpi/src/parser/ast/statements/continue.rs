use crate::{display_prefix, impl_core_display, Display};

pub(crate) struct Continue;

impl Display for Continue {
    fn display(
        &self,
        f: &mut core::fmt::Formatter,
        depth: usize,
        _: bool,
        newline: bool,
    ) -> core::fmt::Result {
        display_prefix!(f, depth);
        write!(f, "Continue")?;

        if newline {
            writeln!(f)
        } else {
            Ok(())
        }
    }
}

impl_core_display!(Continue);
