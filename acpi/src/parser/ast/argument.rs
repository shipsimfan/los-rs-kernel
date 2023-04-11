use super::DataObject;
use crate::parser::{Context, Result, Stream};

pub(crate) enum Argument<'a> {
    DataObject(DataObject<'a>),
}

impl<'a> Argument<'a> {
    pub(super) fn parse(stream: &mut Stream<'a>, context: &mut Context) -> Result<Self> {
        DataObject::parse(stream, context).map(|data_object| Argument::DataObject(data_object))
    }
}

impl<'a> core::fmt::Display for Argument<'a> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Argument::DataObject(data_object) => data_object.fmt(f),
        }
    }
}
