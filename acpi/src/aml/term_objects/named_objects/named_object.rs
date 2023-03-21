use super::{field, op_region};
use crate::{
    aml::{match_next, peek, Context, Result, Stream},
    namespace::Namespace,
};

const EXT_OP_PREFIX: u8 = 0x5B;

const OP_REGION_OP: u8 = 0x80;
const FIELD_OP: u8 = 0x81;

pub(in crate::aml) fn parse(
    stream: &mut Stream,
    namespace: &mut Namespace,
    context: &Context,
) -> Result<bool> {
    match peek!(stream) {
        EXT_OP_PREFIX => {
            stream.next();
            match_next!(stream,
                OP_REGION_OP => op_region::parse(stream, namespace, context)
                FIELD_OP => field::parse(stream, namespace, context)
            )?;
            Ok(true)
        }
        _ => Ok(false),
    }
}
