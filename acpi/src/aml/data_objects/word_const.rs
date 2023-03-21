use crate::{
    aml::{next, Result, Stream},
    namespace::DataType,
};

pub(super) fn parse(stream: &mut Stream) -> Result<DataType> {
    Ok(DataType::Integer(
        u16::from_le_bytes([next!(stream), next!(stream)]) as usize,
    ))
}
