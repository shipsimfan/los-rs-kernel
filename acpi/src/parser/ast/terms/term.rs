use super::{Device, Field, Method, Mutex, Name, OpRegion, Processor, Scope};
use crate::{
    impl_core_display_lifetime,
    parser::{ast::Statement, match_next, Context, Result, Stream},
    Display,
};

pub(crate) enum Term<'a> {
    Device(Device<'a>),
    Field(Field),
    Method(Method<'a>),
    Mutex(Mutex),
    Name(Name<'a>),
    OpRegion(OpRegion<'a>),
    Processor(Processor<'a>),
    Scope(Scope<'a>),
    Statement(Statement<'a>),
}

const NAME_OP: u8 = 0x08;
const SCOPE_OP: u8 = 0x10;
const METHOD_OP: u8 = 0x14;

const EXT_OP_PREFIX: u8 = 0x5B;

const MUTEX_OP: u8 = 0x01;
const OP_REGION_OP: u8 = 0x80;
const FIELD_OP: u8 = 0x81;
const DEVICE_OP: u8 = 0x82;
const PROCESSOR_OP: u8 = 0x83;

impl<'a> Term<'a> {
    pub(super) fn parse(stream: &mut Stream<'a>, context: &mut Context) -> Result<Self> {
        if let Some(statement) = Statement::parse(stream, context)? {
            return Ok(Term::Statement(statement));
        }

        match_next!(stream, "Term",
            METHOD_OP => Method::parse(stream, context).map(|method| Term::Method(method)),
            NAME_OP => Name::parse(stream, context).map(|name| Term::Name(name)),
            SCOPE_OP => Scope::parse(stream, context).map(|scope| Term::Scope(scope)),
            EXT_OP_PREFIX => match_next!(stream, "Extended Term",
                DEVICE_OP => Device::parse(stream, context).map(|device| Term::Device(device)),
                FIELD_OP => Field::parse(stream).map(|field| Term::Field(field)),
                MUTEX_OP => Mutex::parse(stream).map(|mutex| Term::Mutex(mutex)),
                OP_REGION_OP => OpRegion::parse(stream, context).map(|op_region| Term::OpRegion(op_region)),
                PROCESSOR_OP => Processor::parse(stream, context).map(|processor| Term::Processor(processor)),
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
            Term::Mutex(mutex) => mutex.display(f, depth, last),
            Term::Name(name) => name.display(f, depth, last),
            Term::OpRegion(op_region) => op_region.display(f, depth, last),
            Term::Processor(processor) => processor.display(f, depth, last),
            Term::Scope(scope) => scope.display(f, depth, last),
            Term::Statement(statement) => statement.display(f, depth, last),
        }
    }
}

impl_core_display_lifetime!(Term);
