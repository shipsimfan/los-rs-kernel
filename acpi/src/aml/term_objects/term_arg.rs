use crate::{
    aml::{data_objects::data_object, Result, Stream},
    namespace::DataType,
};

pub(super) fn parse(stream: &mut Stream) -> Result<DataType> {
    data_object::parse(stream)
}
