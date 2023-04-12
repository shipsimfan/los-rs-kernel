use super::{Device, Field, Method, Mutex, Name, OpRegion, Processor, Scope};
use crate::{
    impl_core_display_lifetime,
    parser::{ast::Statement, next, Context, Error, Result, Stream},
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
const ELSE_OP: u8 = 0xA1;

impl<'a> Term<'a> {
    pub(super) fn parse(
        stream: &mut Stream<'a>,
        context: &mut Context,
        allow_else: bool,
    ) -> Result<Option<Self>> {
        match next!(stream, "Term") {
            ELSE_OP => {
                if allow_else {
                    stream.prev();
                    return Ok(None);
                } else {
                    return Err(Error::unexpected_byte(ELSE_OP, stream.offset() - 1, "Term"));
                }
            }
            METHOD_OP => Method::parse(stream, context).map(|method| Term::Method(method)),
            NAME_OP => Name::parse(stream, context).map(|name| Term::Name(name)),
            SCOPE_OP => Scope::parse(stream, context).map(|scope| Term::Scope(scope)),
            EXT_OP_PREFIX => match next!(stream, "Extended Term") {
                DEVICE_OP => Device::parse(stream, context).map(|device| Term::Device(device)),
                FIELD_OP => Field::parse(stream).map(|field| Term::Field(field)),
                MUTEX_OP => Mutex::parse(stream).map(|mutex| Term::Mutex(mutex)),
                OP_REGION_OP => {
                    OpRegion::parse(stream, context).map(|op_region| Term::OpRegion(op_region))
                }
                PROCESSOR_OP => {
                    Processor::parse(stream, context).map(|processor| Term::Processor(processor))
                }
                _ => {
                    stream.prev();
                    stream.prev();
                    Statement::parse(stream, context).map(|statement| Term::Statement(statement))
                }
            },
            _ => {
                stream.prev();
                Statement::parse(stream, context).map(|statement| Term::Statement(statement))
            }
        }
        .map(|term| Some(term))
    }

    pub(super) fn parse_methods(&mut self, context: &mut Context) -> Result<()> {
        match self {
            Term::Device(device) => device.parse_methods(context),
            Term::Method(method) => method.parse_methods(context),
            Term::Processor(processor) => processor.parse_methods(context),
            Term::Scope(scope) => scope.parse_methods(context),

            _ => Ok(()),
        }
    }
}

impl<'a> Display for Term<'a> {
    fn display(
        &self,
        f: &mut core::fmt::Formatter,
        depth: usize,
        last: bool,
        newline: bool,
    ) -> core::fmt::Result {
        match self {
            Term::Device(device) => device.display(f, depth, last, newline),
            Term::Field(field) => field.display(f, depth, last, newline),
            Term::Method(method) => method.display(f, depth, last, newline),
            Term::Mutex(mutex) => mutex.display(f, depth, last, newline),
            Term::Name(name) => name.display(f, depth, last, newline),
            Term::OpRegion(op_region) => op_region.display(f, depth, last, newline),
            Term::Processor(processor) => processor.display(f, depth, last, newline),
            Term::Scope(scope) => scope.display(f, depth, last, newline),
            Term::Statement(statement) => statement.display(f, depth, last, newline),
        }
    }
}

impl_core_display_lifetime!(Term);
