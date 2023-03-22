use super::{Device, Field, Method, Mutex, OpRegion};
use crate::aml::{impl_core_display, match_next, Display, Result, Stream};

pub(in crate::aml::term_objects) enum NamedObject {
    Device(Device),
    Field(Field),
    Method(Method),
    Mutex(Mutex),
    OpRegion(OpRegion),
}

const METHOD_OP: u8 = 0x14;

const EXT_OP_PREFIX: u8 = 0x5B;

const MUTEX_OP: u8 = 0x01;
const EVENT_OP: u8 = 0x02;
const OP_REGION_OP: u8 = 0x80;
const FIELD_OP: u8 = 0x81;
const DEVICE_OP: u8 = 0x82;

impl NamedObject {
    pub(in crate::aml::term_objects) fn parse(stream: &mut Stream) -> Result<Self> {
        match_next!(stream,
            METHOD_OP => Method::parse(stream).map(|method| NamedObject::Method(method))

            EXT_OP_PREFIX => match_next!(stream,
                MUTEX_OP => Mutex::parse(stream).map(|mutex| NamedObject::Mutex(mutex))
                EVENT_OP => panic!("TEST")
                OP_REGION_OP => OpRegion::parse(stream).map(|op_region| NamedObject::OpRegion(op_region))
                FIELD_OP => Field::parse(stream).map(|field| NamedObject::Field(field))
                DEVICE_OP => Device::parse(stream).map(|device| NamedObject::Device(device))
            )
        )
    }
}

impl Display for NamedObject {
    fn display(&self, f: &mut core::fmt::Formatter, depth: usize) -> core::fmt::Result {
        match self {
            NamedObject::Device(device) => device.display(f, depth),
            NamedObject::Field(field) => field.display(f, depth),
            NamedObject::Method(method) => method.display(f, depth),
            NamedObject::Mutex(mutex) => mutex.display(f, depth),
            NamedObject::OpRegion(op_region) => op_region.display(f, depth),
        }
    }
}

impl_core_display!(NamedObject);
