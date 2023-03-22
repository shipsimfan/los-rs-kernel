use super::DataObject;
use crate::aml::{impl_core_display, Display, Result, Stream};

pub(in crate::aml::term_objects) enum DataRefObject {
    DataObject(DataObject),
}

impl DataRefObject {
    pub(in crate::aml::term_objects) fn parse(stream: &mut Stream) -> Result<Self> {
        DataObject::parse(stream).map(|data_object| DataRefObject::DataObject(data_object))
    }
}

impl Display for DataRefObject {
    fn display(&self, f: &mut core::fmt::Formatter, depth: usize) -> core::fmt::Result {
        match self {
            DataRefObject::DataObject(data_object) => data_object.display(f, depth),
        }
    }
}

impl_core_display!(DataRefObject);
