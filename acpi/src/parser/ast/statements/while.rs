use crate::{
    display_prefix, impl_core_display_lifetime,
    parser::{
        ast::{Argument, TermList},
        pkg_length, Context, Result, Stream,
    },
    Display,
};

pub(crate) struct While<'a> {
    predicate: Argument<'a>,
    term_list: TermList<'a>,
}

impl<'a> While<'a> {
    pub(super) fn parse(stream: &mut Stream<'a>, context: &mut Context) -> Result<Self> {
        let mut stream = pkg_length::parse_to_stream(stream, "While")?;

        let predicate = Argument::parse(&mut stream, context)?;
        let term_list = TermList::parse(&mut stream, context)?;

        Ok(While {
            predicate,
            term_list,
        })
    }
}

impl<'a> Display for While<'a> {
    fn display(
        &self,
        f: &mut core::fmt::Formatter,
        depth: usize,
        last: bool,
        newline: bool,
    ) -> core::fmt::Result {
        display_prefix!(f, depth);
        write!(f, "While ({}) ", self.predicate)?;
        self.term_list.display(f, depth, last, newline)
    }
}

impl_core_display_lifetime!(While);
