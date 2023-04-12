use super::r#else::Else;
use crate::{
    display_prefix, impl_core_display_lifetime,
    parser::{
        ast::{Argument, TermList},
        pkg_length, Context, Result, Stream,
    },
    Display,
};

pub(crate) struct If<'a> {
    predicate: Argument<'a>,
    term_list: TermList<'a>,
    r#else: Option<Else<'a>>,
}

const ELSE_OP: u8 = 0xA1;

impl<'a> If<'a> {
    pub(super) fn parse(stream: &mut Stream<'a>, context: &mut Context) -> Result<Self> {
        let mut stream = pkg_length::parse_to_stream(stream, "If")?;

        let predicate = Argument::parse(&mut stream, context)?;
        let term_list = TermList::parse_with_else(&mut stream, context)?;

        let r#else = if let Some(c) = stream.next() {
            if c == ELSE_OP {
                Some(Else::parse(&mut stream, context)?)
            } else {
                None
            }
        } else {
            None
        };

        Ok(If {
            predicate,
            term_list,
            r#else,
        })
    }
}

impl<'a> Display for If<'a> {
    fn display(&self, f: &mut core::fmt::Formatter, depth: usize, last: bool) -> core::fmt::Result {
        display_prefix!(f, depth);
        write!(f, "If ({}) ", self.predicate)?;
        self.term_list
            .display(f, depth, last && self.r#else.is_none())?;

        match &self.r#else {
            Some(r#else) => r#else.display(f, depth, last),
            None => Ok(()),
        }
    }
}

impl_core_display_lifetime!(If);
