use super::{AccessField, ExtendedAccessField, NamedField, ReservedField};
use crate::aml::{impl_core_display, peek, Display, Result, Stream};

pub(super) enum FieldElement {
    AccessField(AccessField),
    ExtendedAccessField(ExtendedAccessField),
    NamedField(NamedField),
    ReservedField(ReservedField),
}

const RESERVED_OP: u8 = 0x00;
const ACCESS_OP: u8 = 0x01;
const CONNECT_OP: u8 = 0x02;
const EXTENDED_ACCESS_OP: u8 = 0x03;

impl FieldElement {
    #[allow(unused)]
    pub(super) fn parse(stream: &mut Stream) -> Result<Self> {
        match peek!(stream) {
            RESERVED_OP => {
                stream.next();
                ReservedField::parse(stream)
                    .map(|reserved_field| FieldElement::ReservedField(reserved_field))
            }
            ACCESS_OP => {
                stream.next();
                AccessField::parse(stream)
                    .map(|access_field| FieldElement::AccessField(access_field))
            }
            CONNECT_OP => {
                stream.next();
                todo!()
            }
            EXTENDED_ACCESS_OP => {
                stream.next();
                ExtendedAccessField::parse(stream).map(|extended_access_field| {
                    FieldElement::ExtendedAccessField(extended_access_field)
                })
            }
            _ => NamedField::parse(stream).map(|named_field| FieldElement::NamedField(named_field)),
        }
    }
}

impl Display for FieldElement {
    fn display(&self, f: &mut core::fmt::Formatter, depth: usize, last: bool) -> core::fmt::Result {
        match self {
            FieldElement::AccessField(access_field) => access_field.display(f, depth, last),
            FieldElement::ExtendedAccessField(extended_access_field) => {
                extended_access_field.display(f, depth, last)
            }
            FieldElement::NamedField(named_field) => named_field.display(f, depth, last),
            FieldElement::ReservedField(reserved_field) => reserved_field.display(f, depth, last),
        }
    }
}

impl_core_display!(FieldElement);
