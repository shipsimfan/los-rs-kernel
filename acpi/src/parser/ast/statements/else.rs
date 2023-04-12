use crate::{
    display_prefix, impl_core_display_lifetime,
    parser::{ast::TermList, pkg_length, Context, Result, Stream},
    Display,
};

pub(crate) struct Else<'a> {
    term_list: TermList<'a>,
}

impl<'a> Else<'a> {
    pub(super) fn parse(stream: &mut Stream<'a>, context: &mut Context) -> Result<Self> {
        let mut stream = pkg_length::parse_to_stream(stream, "Else")?;

        let term_list = TermList::parse(&mut stream, context)?;

        Ok(Else { term_list })
    }
}

impl<'a> Display for Else<'a> {
    fn display(
        &self,
        f: &mut core::fmt::Formatter,
        depth: usize,
        last: bool,
        newline: bool,
    ) -> core::fmt::Result {
        display_prefix!(f, depth);
        write!(f, "Else ")?;
        self.term_list.display(f, depth, last, newline)
    }
}

impl_core_display_lifetime!(Else);
