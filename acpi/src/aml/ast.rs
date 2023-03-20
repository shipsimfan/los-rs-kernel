use super::{ByteStream, Result, TermList};

pub(super) trait ASTNode {
    fn display(&self, f: &mut core::fmt::Formatter, depth: usize) -> core::fmt::Result;

    fn prefix_depth(&self, f: &mut core::fmt::Formatter, depth: usize) -> core::fmt::Result {
        for _ in 0..depth {
            write!(f, "  ")?;
        }
        Ok(())
    }
}

pub(crate) struct AML {
    term_list: TermList,
}

impl<'a: 'b, 'b> AML {
    pub(crate) fn parse(definition_block: &'a [u8]) -> Result<Self> {
        let mut stream = ByteStream::new(definition_block);

        let term_list = TermList::parse(&mut stream)?;

        Ok(AML { term_list })
    }
}

impl ASTNode for AML {
    fn display(&self, f: &mut core::fmt::Formatter, depth: usize) -> core::fmt::Result {
        self.prefix_depth(f, depth)?;
        writeln!(f, "AML:")?;
        self.term_list.display(f, depth + 1)
    }
}

impl core::fmt::Display for AML {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.display(f, 0)
    }
}
