use super::TermObj;
use crate::aml::{impl_core_display, Display, Result, Stream};
use alloc::vec::Vec;

pub(in crate::aml) struct TermList {
    list: Vec<TermObj>,
}

impl TermList {
    pub(in crate::aml) fn parse(stream: &mut Stream) -> Result<Self> {
        let mut list = Vec::new();
        while stream.peek().is_some() {
            list.push(TermObj::parse(stream)?);
        }

        Ok(TermList { list })
    }
}

impl Display for TermList {
    fn display(&self, f: &mut core::fmt::Formatter, depth: usize, last: bool) -> core::fmt::Result {
        if self.list.len() == 0 {
            return writeln!(f, "{{}}");
        }

        writeln!(f, "{{")?;

        for i in 0..self.list.len() {
            self.list[i].display(f, depth + 1, i == self.list.len() - 1)?;
        }

        self.display_prefix(f, depth)?;
        writeln!(f, "}}")?;

        if last {
            Ok(())
        } else {
            writeln!(f)
        }
    }
}

impl_core_display!(TermList);
