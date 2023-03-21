use crate::{
    aml::{data_objects::word_const, match_next, Result, Stream},
    namespace::DataType,
};

const ONE_OP: u8 = 0x01;
const WORD_PREFIX: u8 = 0x0B;

pub(super) fn parse(stream: &mut Stream) -> Result<DataType> {
    match_next!(stream,
        ONE_OP => Ok(DataType::Integer(1))
        WORD_PREFIX => word_const::parse(stream)
    )
}
