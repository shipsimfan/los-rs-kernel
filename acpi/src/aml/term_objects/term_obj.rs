use super::Object;
use crate::aml::{impl_core_display, Display, Result, Stream};

pub(super) enum TermObj {
    Object(Object),
}

impl TermObj {
    pub(super) fn parse(stream: &mut Stream) -> Result<Self> {
        Object::parse(stream).map(|object| TermObj::Object(object))
    }
}

impl Display for TermObj {
    fn display(&self, f: &mut core::fmt::Formatter, depth: usize, last: bool) -> core::fmt::Result {
        match self {
            TermObj::Object(object) => object.display(f, depth, last),
        }
    }
}

impl_core_display!(TermObj);
