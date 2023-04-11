use super::SimpleName;
use crate::parser::{ast::expressions::ReferenceTypeOp, next, Context, Result, Stream};

pub(crate) enum SuperName<'a> {
    Debug,
    ReferenceTypeOp(ReferenceTypeOp<'a>),
    Simple(SimpleName),
}

impl<'a> SuperName<'a> {
    pub(in crate::parser) fn parse(stream: &mut Stream<'a>, context: &mut Context) -> Result<Self> {
        if let Some(reference_type_op) = ReferenceTypeOp::parse(stream, context)? {
            return Ok(SuperName::ReferenceTypeOp(reference_type_op));
        }

        let c = next!(stream, "Super Name");
        if c == 0x5B {
            let c = next!(stream, "Super Name");

            if c == 0x31 {
                return Ok(SuperName::Debug);
            }

            stream.prev();
        }

        stream.prev();

        SimpleName::parse(stream).map(|simple_name| SuperName::Simple(simple_name))
    }
}

impl<'a> core::fmt::Display for SuperName<'a> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            SuperName::Debug => write!(f, "Debug"),
            SuperName::ReferenceTypeOp(reference_type_op) => reference_type_op.fmt(f),
            SuperName::Simple(simple_name) => simple_name.fmt(f),
        }
    }
}
