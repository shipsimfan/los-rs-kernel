pub(super) trait Display {
    fn display(&self, f: &mut core::fmt::Formatter, depth: usize) -> core::fmt::Result;

    fn display_prefix(&self, f: &mut core::fmt::Formatter, depth: usize) -> core::fmt::Result {
        for _ in 0..depth {
            write!(f, "  ")?;
        }
        Ok(())
    }
}
