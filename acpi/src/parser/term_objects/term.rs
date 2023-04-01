use super::{Device, Field, Method, Mutex, OpRegion, Scope};
use crate::parser::{
    match_next, Result, Stream, DEVICE_OP, EXT_OP_PREFIX, FIELD_OP, METHOD_OP, MUTEX_OP,
    OP_REGION_OP, SCOPE_OP,
};

pub(crate) enum Term<'a> {
    Device(Device<'a>),
    Field(Field<'a>),
    Method(Method<'a>),
    Mutex(Mutex),
    OpRegion(OpRegion),
    Scope(Scope<'a>),
}

impl<'a> Term<'a> {
    pub(super) fn parse(stream: &mut Stream<'a>) -> Result<Self> {
        match_next!(stream,
            METHOD_OP => Method::parse(stream).map(|method| Term::Method(method))
            SCOPE_OP => Scope::parse(stream).map(|scope| Term::Scope(scope))
            EXT_OP_PREFIX => match_next!(stream,
                DEVICE_OP => Device::parse(stream).map(|device| Term::Device(device))
                FIELD_OP => Field::parse(stream).map(|field| Term::Field(field))
                MUTEX_OP => Mutex::parse(stream).map(|mutex| Term::Mutex(mutex))
                OP_REGION_OP => OpRegion::parse(stream).map(|op_region| Term::OpRegion(op_region))
            )
        )
    }
}
