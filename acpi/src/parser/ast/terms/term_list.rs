use super::Term;
use crate::{
    display_prefix, impl_core_display,
    parser::{Context, Result, Stream},
    Display,
};
use alloc::vec::Vec;

pub(crate) struct TermList {
    terms: Vec<Term>,
}

impl TermList {
    pub(in crate::parser::ast) fn parse(
        stream: &mut Stream,
        context: &mut Context,
    ) -> Result<Self> {
        todo!()
    }
}

impl Display for TermList {
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

impl_core_display!(TermList);
