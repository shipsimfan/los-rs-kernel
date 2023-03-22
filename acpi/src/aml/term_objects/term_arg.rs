use super::DataObject;
use crate::aml::{impl_core_display, Display, Result, Stream};

pub(super) enum TermArg {
    DataObject(DataObject),
}

impl TermArg {
    pub(super) fn parse(stream: &mut Stream) -> Result<Self> {
        DataObject::parse(stream).map(|data_object| TermArg::DataObject(data_object))
    }
}

impl Display for TermArg {
    fn display(&self, f: &mut core::fmt::Formatter, depth: usize) -> core::fmt::Result {
        match self {
            TermArg::DataObject(data_object) => data_object.display(f, depth),
        }
    }
}

impl_core_display!(TermArg);
