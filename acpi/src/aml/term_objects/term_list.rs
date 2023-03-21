use super::TermObj;
use crate::aml::{impl_core_display, Display, Result, Stream};
use alloc::vec::Vec;

pub(in crate::aml) struct TermList {
    offset: usize,
    list: Vec<TermObj>,
}

impl TermList {
    pub(in crate::aml) fn parse(stream: &mut Stream) -> Result<Self> {
        let offset = stream.offset();

        let mut list = Vec::new();
        while stream.peek().is_some() {
            list.push(TermObj::parse(stream)?);
        }

        Ok(TermList { offset, list })
    }
}

impl Display for TermList {
    fn display(&self, f: &mut core::fmt::Formatter, depth: usize) -> core::fmt::Result {
        self.display_prefix(f, depth)?;
        writeln!(f, "Term List @ {}:", self.offset)?;

        for term_obj in &self.list {
            term_obj.display(f, depth + 1)?;
        }

        Ok(())
    }
}

impl_core_display!(TermList);
