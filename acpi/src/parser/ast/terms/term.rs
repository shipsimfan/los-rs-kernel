use super::{
    Alias, BankField, CreateBitField, CreateByteField, CreateDWordField, CreateField,
    CreateQWordField, CreateWordField, DataRegion, Device, External, Field, Method, Mutex, Name,
    OpRegion, PowerRes, Processor, Scope,
};
use crate::{
    impl_core_display_lifetime,
    parser::{ast::Statement, next, Context, Result, Stream},
    Display,
};

pub(crate) enum Term<'a> {
    Alias(Alias),
    BankField(BankField<'a>),
    CreateBitField(CreateBitField<'a>),
    CreateByteField(CreateByteField<'a>),
    CreateDWordField(CreateDWordField<'a>),
    CreateField(CreateField<'a>),
    CreateQWordField(CreateQWordField<'a>),
    CreateWordField(CreateWordField<'a>),
    DataRegion(DataRegion<'a>),
    Device(Device<'a>),
    External(External),
    Field(Field),
    Method(Method<'a>),
    Mutex(Mutex),
    Name(Name<'a>),
    OpRegion(OpRegion<'a>),
    PowerRes(PowerRes<'a>),
    Processor(Processor<'a>),
    Scope(Scope<'a>),
    Statement(Statement<'a>),
}

const ALIAS_OP: u8 = 0x06;
const NAME_OP: u8 = 0x08;
const SCOPE_OP: u8 = 0x10;
const METHOD_OP: u8 = 0x14;
const EXTERNAL_OP: u8 = 0x15;
const CREATE_DWORD_FIELD_OP: u8 = 0x8A;
const CREATE_WORD_FIELD_OP: u8 = 0x8B;
const CREATE_BYTE_FIELD_OP: u8 = 0x8C;
const CREATE_BIT_FIELD_OP: u8 = 0x8D;
const CREATE_QWORD_FIELD_OP: u8 = 0x8F;

const EXT_OP_PREFIX: u8 = 0x5B;

const MUTEX_OP: u8 = 0x01;
const CREATE_FIELD_OP: u8 = 0x13;
const OP_REGION_OP: u8 = 0x80;
const FIELD_OP: u8 = 0x81;
const DEVICE_OP: u8 = 0x82;
const PROCESSOR_OP: u8 = 0x83;
const POWER_RES_OP: u8 = 0x84;
const BANK_FIELD_OP: u8 = 0x87;
const DATA_REGION_OP: u8 = 0x88;

impl<'a> Term<'a> {
    pub(super) fn parse(stream: &mut Stream<'a>, context: &mut Context) -> Result<Option<Self>> {
        match next!(stream, "Term") {
            ALIAS_OP => Alias::parse(stream).map(|alias| Term::Alias(alias)),
            CREATE_BIT_FIELD_OP => CreateBitField::parse(stream, context)
                .map(|create_bit_field| Term::CreateBitField(create_bit_field)),
            CREATE_BYTE_FIELD_OP => CreateByteField::parse(stream, context)
                .map(|create_byte_field| Term::CreateByteField(create_byte_field)),
            CREATE_DWORD_FIELD_OP => CreateDWordField::parse(stream, context)
                .map(|create_dword_field| Term::CreateDWordField(create_dword_field)),
            CREATE_QWORD_FIELD_OP => CreateQWordField::parse(stream, context)
                .map(|create_qword_field| Term::CreateQWordField(create_qword_field)),
            CREATE_WORD_FIELD_OP => CreateWordField::parse(stream, context)
                .map(|create_word_field| Term::CreateWordField(create_word_field)),
            EXTERNAL_OP => {
                External::parse(stream, context).map(|external| Term::External(external))
            }
            METHOD_OP => Method::parse(stream, context).map(|method| Term::Method(method)),
            NAME_OP => Name::parse(stream, context).map(|name| Term::Name(name)),
            SCOPE_OP => Scope::parse(stream, context).map(|scope| Term::Scope(scope)),
            EXT_OP_PREFIX => match next!(stream, "Extended Term") {
                BANK_FIELD_OP => {
                    BankField::parse(stream, context).map(|bank_field| Term::BankField(bank_field))
                }
                CREATE_FIELD_OP => CreateField::parse(stream, context)
                    .map(|create_field| Term::CreateField(create_field)),
                DATA_REGION_OP => DataRegion::parse(stream, context)
                    .map(|data_region| Term::DataRegion(data_region)),
                DEVICE_OP => Device::parse(stream, context).map(|device| Term::Device(device)),
                FIELD_OP => Field::parse(stream).map(|field| Term::Field(field)),
                MUTEX_OP => Mutex::parse(stream).map(|mutex| Term::Mutex(mutex)),
                OP_REGION_OP => {
                    OpRegion::parse(stream, context).map(|op_region| Term::OpRegion(op_region))
                }
                POWER_RES_OP => {
                    PowerRes::parse(stream, context).map(|power_res| Term::PowerRes(power_res))
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
            Term::Alias(alias) => alias.display(f, depth, last, newline),
            Term::BankField(bank_field) => bank_field.display(f, depth, last, newline),
            Term::CreateBitField(create_bit_field) => {
                create_bit_field.display(f, depth, last, newline)
            }
            Term::CreateByteField(create_byte_field) => {
                create_byte_field.display(f, depth, last, newline)
            }
            Term::CreateDWordField(create_dword_field) => {
                create_dword_field.display(f, depth, last, newline)
            }
            Term::CreateField(create_field) => create_field.display(f, depth, last, newline),
            Term::CreateQWordField(create_qword_field) => {
                create_qword_field.display(f, depth, last, newline)
            }
            Term::CreateWordField(create_word_field) => {
                create_word_field.display(f, depth, last, newline)
            }
            Term::DataRegion(data_region) => data_region.display(f, depth, last, newline),
            Term::Device(device) => device.display(f, depth, last, newline),
            Term::External(external) => external.display(f, depth, last, newline),
            Term::Field(field) => field.display(f, depth, last, newline),
            Term::Method(method) => method.display(f, depth, last, newline),
            Term::Mutex(mutex) => mutex.display(f, depth, last, newline),
            Term::Name(name) => name.display(f, depth, last, newline),
            Term::OpRegion(op_region) => op_region.display(f, depth, last, newline),
            Term::PowerRes(power_res) => power_res.display(f, depth, last, newline),
            Term::Processor(processor) => processor.display(f, depth, last, newline),
            Term::Scope(scope) => scope.display(f, depth, last, newline),
            Term::Statement(statement) => statement.display(f, depth, last, newline),
        }
    }
}

impl_core_display_lifetime!(Term);
