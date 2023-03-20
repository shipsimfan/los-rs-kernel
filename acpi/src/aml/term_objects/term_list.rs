use super::TermObj;
use crate::aml::{ASTNode, ByteStream, Result};
use alloc::vec::Vec;

pub(in crate::aml) struct TermList {
    list: Vec<TermObj>,
    offset: usize,
}

impl TermList {
    pub(in crate::aml) fn parse(stream: &mut ByteStream) -> Result<Self> {
        let mut list = Vec::new();
        let offset = stream.offset();

        while stream.peek().is_some() {
            let term_obj = TermObj::parse(stream)?;
            list.push(term_obj);
        }

        Ok(TermList { list, offset })
    }
}

impl ASTNode for TermList {
    fn display(&self, f: &mut core::fmt::Formatter, depth: usize) -> core::fmt::Result {
        self.prefix_depth(f, depth)?;
        writeln!(f, "{} | Term List:", self.offset)?;

        for object in &self.list {
            object.display(f, depth + 1)?;
        }

        Ok(())
    }
}
