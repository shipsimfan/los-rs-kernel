use super::DataObject;
use crate::aml::{impl_core_display, Display, Error, Result, Stream};

pub(super) enum TermArg {
    DataObject(DataObject),
}

impl TermArg {
    pub(super) fn parse(stream: &mut Stream) -> Result<Self> {
        match DataObject::parse(stream)? {
            Some(data_object) => Ok(TermArg::DataObject(data_object)),
            None => Err(Error::unexpected_byte(
                stream.next().unwrap(),
                stream.offset() - 1,
            )),
        }
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
