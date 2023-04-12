use super::TermList;
use crate::{
    display_prefix, impl_core_display_lifetime,
    parser::{name_string, pkg_length, Context, Result, Stream},
    Display, Path,
};

pub(crate) struct Scope<'a> {
    path: Path,
    term_list: TermList<'a>,
}

impl<'a> Scope<'a> {
    pub(super) fn parse(stream: &mut Stream<'a>, context: &mut Context) -> Result<Self> {
        let mut stream = pkg_length::parse_to_stream(stream, "Scope")?;

        let path = name_string::parse(&mut stream, "Scope")?;

        context.push_path(&path);
        let term_list = TermList::parse(&mut stream, context)?;
        context.pop_path();

        Ok(Scope { path, term_list })
    }

    pub(super) fn parse_methods(&mut self, context: &mut Context) -> Result<()> {
        self.term_list.parse_methods(context)
    }
}

impl<'a> Display for Scope<'a> {
    fn display(
        &self,
        f: &mut core::fmt::Formatter,
        depth: usize,
        last: bool,
        newline: bool,
    ) -> core::fmt::Result {
        display_prefix!(f, depth);
        write!(f, "Scope ({}) ", self.path)?;
        self.term_list.display(f, depth, last, newline)
    }
}

impl_core_display_lifetime!(Scope);
