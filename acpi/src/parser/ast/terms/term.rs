use super::{Device, Field, Method, Name, OpRegion, Scope};
use crate::{
    impl_core_display_lifetime,
    parser::{match_next, Context, Result, Stream},
    Display,
};

pub(crate) enum Term<'a> {
    Device(Device<'a>),
    Field(Field),
    Method(Method<'a>),
    Name(Name<'a>),
    OpRegion(OpRegion<'a>),
    Scope(Scope<'a>),
}

const NAME_OP: u8 = 0x08;
const SCOPE_OP: u8 = 0x10;
const METHOD_OP: u8 = 0x14;

const EXT_OP_PREFIX: u8 = 0x5B;

const OP_REGION_OP: u8 = 0x80;
const FIELD_OP: u8 = 0x81;
const DEVICE_OP: u8 = 0x82;

impl<'a> Term<'a> {
    pub(super) fn parse(stream: &mut Stream<'a>, context: &mut Context) -> Result<Self> {
        match_next!(stream, "Term",
            METHOD_OP => Method::parse(stream, context).map(|method| Term::Method(method)),
            NAME_OP => Name::parse(stream, context).map(|name| Term::Name(name)),
            SCOPE_OP => Scope::parse(stream, context).map(|scope| Term::Scope(scope)),
            EXT_OP_PREFIX => match_next!(stream, "Extended Term",
                DEVICE_OP => Device::parse(stream, context).map(|device| Term::Device(device)),
                FIELD_OP => Field::parse(stream).map(|field| Term::Field(field)),
                OP_REGION_OP => OpRegion::parse(stream, context).map(|op_region| Term::OpRegion(op_region)),
            ),
        )
    }
}

impl<'a> Display for Term<'a> {
    fn display(&self, f: &mut core::fmt::Formatter, depth: usize, last: bool) -> core::fmt::Result {
        match self {
            Term::Device(device) => device.display(f, depth, last),
            Term::Field(field) => field.display(f, depth, last),
            Term::Method(method) => method.display(f, depth, last),
            Term::Name(name) => name.display(f, depth, last),
            Term::OpRegion(op_region) => op_region.display(f, depth, last),
            Term::Scope(scope) => scope.display(f, depth, last),
        }
    }
}

impl_core_display_lifetime!(Term);
