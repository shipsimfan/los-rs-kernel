use super::Method;
use crate::aml::{impl_core_display, match_next, Display, Result, Stream};

pub(in crate::aml::term_objects) enum NamedObject {
    Method(Method),
}

const METHOD_OP: u8 = 0x14;

impl NamedObject {
    pub(in crate::aml::term_objects) fn parse(stream: &mut Stream) -> Result<Self> {
        match_next!(stream,
            METHOD_OP => Method::parse(stream).map(|method| NamedObject::Method(method))
        )
    }
}

impl Display for NamedObject {
    fn display(&self, f: &mut core::fmt::Formatter, depth: usize) -> core::fmt::Result {
        match self {
            NamedObject::Method(method) => method.display(f, depth),
        }
    }
}

impl_core_display!(NamedObject);
