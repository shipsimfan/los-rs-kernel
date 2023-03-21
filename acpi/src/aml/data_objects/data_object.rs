use super::computational_data;
use crate::{
    aml::{Result, Stream},
    namespace::DataType,
};

pub(in crate::aml) fn parse(stream: &mut Stream) -> Result<DataType> {
    computational_data::parse(stream)
}
