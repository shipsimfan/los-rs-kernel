use crate::aml::{ASTNode, ByteStream, NameString, PkgLength, Result, TermList};

pub struct DefScope {
    name: NameString,
    term_list: TermList,
    offset: usize,
}

impl DefScope {
    pub(super) fn parse(stream: &mut ByteStream) -> Result<Self> {
        let offset = stream.offset() - 1;

        let length = PkgLength::parse(stream)?;
        let mut stream = ByteStream::new(stream.collect(length)?);

        let name = NameString::parse(&mut stream)?;
        let term_list = TermList::parse(&mut stream)?;

        Ok(DefScope {
            name,
            term_list,
            offset,
        })
    }
}

impl ASTNode for DefScope {
    fn display(&self, f: &mut core::fmt::Formatter, depth: usize) -> core::fmt::Result {
        self.prefix_depth(f, depth)?;
        writeln!(f, "{} | Scope \"{}\":", self.offset, self.name)?;
        self.term_list.display(f, depth + 1)
    }
}
