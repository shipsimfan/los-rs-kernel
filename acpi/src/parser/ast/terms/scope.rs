use crate::{
    display_prefix, impl_core_display,
    parser::{name_string, pkg_length, Context, Result, Stream},
    Display, Path,
};

pub(crate) struct Scope {
    path: Path,
    // TODO: Add TermList
}

impl Scope {
    pub(super) fn parse(stream: &mut Stream, _: &mut Context) -> Result<Self> {
        let mut stream = pkg_length::parse_to_stream(stream, "Scope")?;

        let path = name_string::parse(&mut stream, "Scope")?;

        Ok(Scope { path })
    }
}

impl Display for Scope {
    fn display(&self, f: &mut core::fmt::Formatter, depth: usize, _: bool) -> core::fmt::Result {
        display_prefix!(f, depth);
        writeln!(f, "Scope ({})", self.path)
    }
}

impl_core_display!(Scope);
