use super::Term;
use crate::{
    display_prefix, impl_core_display_lifetime,
    parser::{Context, Result, Stream},
    Display,
};
use alloc::vec::Vec;

pub(crate) struct TermList<'a> {
    terms: Vec<Term<'a>>,
}

impl<'a> TermList<'a> {
    pub(in crate::parser::ast) fn parse(
        stream: &mut Stream<'a>,
        context: &mut Context,
    ) -> Result<Self> {
        let mut terms = Vec::new();

        while stream.remaining() > 0 {
            terms.push(Term::parse(stream, context)?);
        }

        Ok(TermList { terms })
    }
}

impl<'a> Display for TermList<'a> {
    fn display(&self, f: &mut core::fmt::Formatter, depth: usize, last: bool) -> core::fmt::Result {
        write!(f, "{{")?;

        if self.terms.len() == 0 {
            return writeln!(f, " }}");
        }

        writeln!(f)?;

        for i in 0..self.terms.len() {
            self.terms[i].display(f, depth + 1, i == self.terms.len() - 1)?;
        }

        display_prefix!(f, depth);
        writeln!(f, "}}")?;

        if !last {
            writeln!(f)
        } else {
            Ok(())
        }
    }
}

impl_core_display_lifetime!(TermList);
