use crate::parser::{DataObject, Result, Stream};

pub(crate) enum Argument {
    DataObject(DataObject),
}

impl Argument {
    pub(in crate::parser) fn parse(stream: &mut Stream) -> Result<Self> {
        DataObject::parse(stream).map(|data_object| Argument::DataObject(data_object))
    }
}

impl core::fmt::Display for Argument {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Argument::DataObject(data_object) => data_object.fmt(f),
        }
    }
}
