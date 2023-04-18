use crate::{display_prefix, impl_core_display, Display};

pub(crate) struct BreakPoint;

impl Display for BreakPoint {
    fn display(
        &self,
        f: &mut core::fmt::Formatter,
        depth: usize,
        _: bool,
        newline: bool,
    ) -> core::fmt::Result {
        display_prefix!(f, depth);
        write!(f, "BreakPoint")?;

        if newline {
            writeln!(f)
        } else {
            Ok(())
        }
    }
}

impl_core_display!(BreakPoint);
