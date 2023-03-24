use super::{ComputationalData, Package, VarPackage};
use crate::aml::{impl_core_display, peek, Display, Result, Stream};

pub(in crate::aml::term_objects) enum DataObject {
    ComputationalData(ComputationalData),
    Package(Package),
    VarPackage(VarPackage),
}

const PACKAGE_OP: u8 = 0x12;
const VAR_PACKAGE_OP: u8 = 0x13;

impl DataObject {
    pub(in crate::aml::term_objects) fn parse(stream: &mut Stream) -> Result<Option<Self>> {
        match peek!(stream) {
            PACKAGE_OP => {
                stream.next();
                Package::parse(stream).map(|package| Some(DataObject::Package(package)))
            }
            VAR_PACKAGE_OP => {
                stream.next();
                VarPackage::parse(stream)
                    .map(|var_package| Some(DataObject::VarPackage(var_package)))
            }
            _ => ComputationalData::parse(stream).map(|computational_data| {
                computational_data
                    .map(|computational_data| DataObject::ComputationalData(computational_data))
            }),
        }
    }
}

impl Display for DataObject {
    fn display(&self, f: &mut core::fmt::Formatter, depth: usize, last: bool) -> core::fmt::Result {
        match self {
            DataObject::ComputationalData(computational_data) => {
                computational_data.display(f, depth, last)
            }
            DataObject::Package(package) => package.display(f, depth, last),
            DataObject::VarPackage(var_package) => var_package.display(f, depth, last),
        }
    }
}

impl_core_display!(DataObject);
