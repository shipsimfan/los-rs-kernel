use super::{
    CreateBitField, CreateByteField, CreateDWordField, CreateField, CreateQWordField,
    CreateWordField, DataRegion, Device, Event, External, Field, IndexField, Method, Mutex,
    OpRegion, PowerRes, Processor, ThermalZone,
};
use crate::aml::{
    impl_core_display, match_next, term_objects::named_objects::BankField, Display, Result, Stream,
};

pub(in crate::aml::term_objects) enum NamedObject {
    BankField(BankField),
    CreateBitField(CreateBitField),
    CreateByteField(CreateByteField),
    CreateDWordField(CreateDWordField),
    CreateField(CreateField),
    CreateQWordField(CreateQWordField),
    CreateWordField(CreateWordField),
    Device(Device),
    DataRegion(DataRegion),
    Event(Event),
    External(External),
    Field(Field),
    IndexField(IndexField),
    Method(Method),
    Mutex(Mutex),
    OpRegion(OpRegion),
    PowerRes(PowerRes),
    Processor(Processor),
    ThermalZone(ThermalZone),
}

const METHOD_OP: u8 = 0x14;
const EXTERNAL_OP: u8 = 0x15;
const CREATE_DWORD_FIELD_OP: u8 = 0x8A;
const CREATE_WORD_FIELD: u8 = 0x8B;
const CREATE_BYTE_FIELD_OP: u8 = 0x8C;
const CREATE_BIT_FIELD_OP: u8 = 0x8D;
const CREATE_QWORD_FIELD_OP: u8 = 0x8F;

const EXT_OP_PREFIX: u8 = 0x5B;

const MUTEX_OP: u8 = 0x01;
const EVENT_OP: u8 = 0x02;
const CREATE_FIELD_OP: u8 = 0x13;
const OP_REGION_OP: u8 = 0x80;
const FIELD_OP: u8 = 0x81;
const DEVICE_OP: u8 = 0x82;
const PROCESSOR_OP: u8 = 0x83;
const POWER_RES_OP: u8 = 0x84;
const THERMAL_ZONE_OP: u8 = 0x85;
const INDEX_FIELD_OP: u8 = 0x86;
const BANK_FIELD_OP: u8 = 0x87;
const DATA_REGION_OP: u8 = 0x88;

impl NamedObject {
    pub(in crate::aml::term_objects) fn parse(stream: &mut Stream) -> Result<Self> {
        match_next!(stream,
            METHOD_OP => Method::parse(stream).map(|method| NamedObject::Method(method))
            EXTERNAL_OP => External::parse(stream).map(|external| NamedObject::External(external))
            CREATE_DWORD_FIELD_OP => CreateDWordField::parse(stream).map(|create_dword_field| NamedObject::CreateDWordField(create_dword_field))
            CREATE_WORD_FIELD => CreateWordField::parse(stream).map(|create_word_field| NamedObject::CreateWordField(create_word_field))
            CREATE_BYTE_FIELD_OP => CreateByteField::parse(stream).map(|create_byte_field| NamedObject::CreateByteField(create_byte_field))
            CREATE_BIT_FIELD_OP => CreateBitField::parse(stream).map(|create_bit_field| NamedObject::CreateBitField(create_bit_field))
            CREATE_QWORD_FIELD_OP => CreateQWordField::parse(stream).map(|create_qword_field| NamedObject::CreateQWordField(create_qword_field))

            EXT_OP_PREFIX => match_next!(stream,
                MUTEX_OP => Mutex::parse(stream).map(|mutex| NamedObject::Mutex(mutex))
                EVENT_OP => Event::parse(stream).map(|event| NamedObject::Event(event))
                CREATE_FIELD_OP => CreateField::parse(stream).map(|create_field| NamedObject::CreateField(create_field))
                OP_REGION_OP => OpRegion::parse(stream).map(|op_region| NamedObject::OpRegion(op_region))
                FIELD_OP => Field::parse(stream).map(|field| NamedObject::Field(field))
                DEVICE_OP => Device::parse(stream).map(|device| NamedObject::Device(device))
                PROCESSOR_OP => Processor::parse(stream).map(|processor| NamedObject::Processor(processor))
                POWER_RES_OP => PowerRes::parse(stream).map(|power_res| NamedObject::PowerRes(power_res))
                THERMAL_ZONE_OP => ThermalZone::parse(stream).map(|thermal_zone| NamedObject::ThermalZone(thermal_zone))
                INDEX_FIELD_OP => IndexField::parse(stream).map(|index_field| NamedObject::IndexField(index_field))
                BANK_FIELD_OP => BankField::parse(stream).map(|bank_field| NamedObject::BankField(bank_field))
                DATA_REGION_OP => DataRegion::parse(stream).map(|data_region| NamedObject::DataRegion(data_region))
            )
        )
    }
}

impl Display for NamedObject {
    fn display(&self, f: &mut core::fmt::Formatter, depth: usize, last: bool) -> core::fmt::Result {
        match self {
            NamedObject::BankField(bank_field) => bank_field.display(f, depth, last),
            NamedObject::CreateBitField(create_bit_field) => {
                create_bit_field.display(f, depth, last)
            }
            NamedObject::CreateByteField(create_byte_field) => {
                create_byte_field.display(f, depth, last)
            }
            NamedObject::CreateDWordField(create_dword_field) => {
                create_dword_field.display(f, depth, last)
            }
            NamedObject::CreateField(create_field) => create_field.display(f, depth, last),
            NamedObject::CreateQWordField(create_qword_field) => {
                create_qword_field.display(f, depth, last)
            }
            NamedObject::CreateWordField(create_word_field) => {
                create_word_field.display(f, depth, last)
            }
            NamedObject::Device(device) => device.display(f, depth, last),
            NamedObject::DataRegion(data_region) => data_region.display(f, depth, last),
            NamedObject::Event(event) => event.display(f, depth, last),
            NamedObject::External(external) => external.display(f, depth, last),
            NamedObject::Field(field) => field.display(f, depth, last),
            NamedObject::IndexField(index_field) => index_field.display(f, depth, last),
            NamedObject::Method(method) => method.display(f, depth, last),
            NamedObject::Mutex(mutex) => mutex.display(f, depth, last),
            NamedObject::OpRegion(op_region) => op_region.display(f, depth, last),
            NamedObject::PowerRes(power_res) => power_res.display(f, depth, last),
            NamedObject::Processor(processor) => processor.display(f, depth, last),
            NamedObject::ThermalZone(thermal_zone) => thermal_zone.display(f, depth, last),
        }
    }
}

impl_core_display!(NamedObject);
