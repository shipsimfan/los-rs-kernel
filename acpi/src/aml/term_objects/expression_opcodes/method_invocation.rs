use crate::aml::{impl_core_display, term_objects::TermArg, Display, NameString, Result, Stream};
use alloc::vec::Vec;

pub(in crate::aml::term_objects) struct MethodInvocation {
    name: NameString,
    term_args: Vec<TermArg>,
}

impl MethodInvocation {
    pub(super) fn parse(stream: &mut Stream) -> Result<Self> {
        let name = NameString::parse(stream)?;

        let mut term_args = Vec::new();
        while stream.peek().is_some() {
            term_args.push(TermArg::parse(stream)?);
        }

        Ok(MethodInvocation { name, term_args })
    }
}

impl Display for MethodInvocation {
    fn display(&self, f: &mut core::fmt::Formatter, depth: usize, _: bool) -> core::fmt::Result {
        self.display_prefix(f, depth)?;
        write!(f, "{} (", self.name)?;

        for i in 0..self.term_args.len() {
            write!(f, "{}", self.term_args[i])?;

            if i != self.term_args.len() - 1 {
                write!(f, ", ")?;
            }
        }

        write!(f, ")")
    }
}

impl_core_display!(MethodInvocation);
