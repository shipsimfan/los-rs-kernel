use super::Object;
use crate::aml::{ASTNode, ByteStream, Result};

pub(super) enum TermObj {
    Object(Object),
}

impl TermObj {
    pub(super) fn parse(stream: &mut ByteStream) -> Result<Self> {
        let c = stream.next().unwrap();

        Object::parse(stream, c).map(|object| TermObj::Object(object))
    }
}

impl ASTNode for TermObj {
    fn display(&self, f: &mut core::fmt::Formatter, depth: usize) -> core::fmt::Result {
        match self {
            TermObj::Object(object) => object.display(f, depth),
        }
    }
}
