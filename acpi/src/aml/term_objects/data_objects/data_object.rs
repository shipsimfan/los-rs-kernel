use super::{ComputationalData, Package};
use crate::aml::{impl_core_display, peek, Display, Result, Stream};

pub(in crate::aml::term_objects) enum DataObject {
    ComputationalData(ComputationalData),
    Package(Package),
}

const PACKAGE_OP: u8 = 0x12;

impl DataObject {
    pub(in crate::aml::term_objects) fn parse(stream: &mut Stream) -> Result<Self> {
        match peek!(stream) {
            PACKAGE_OP => {
                stream.next();
                Package::parse(stream).map(|package| DataObject::Package(package))
            }
            _ => ComputationalData::parse(stream)
                .map(|computational_data| DataObject::ComputationalData(computational_data)),
        }
    }
}

impl Display for DataObject {
    fn display(&self, f: &mut core::fmt::Formatter, depth: usize) -> core::fmt::Result {
        match self {
            DataObject::ComputationalData(computational_data) => {
                computational_data.display(f, depth)
            }
            DataObject::Package(package) => package.display(f, depth),
        }
    }
}

impl_core_display!(DataObject);
