use super::{field_list::FieldList, FieldFlags};
use crate::aml::{
    impl_core_display, pkg_length, term_objects::TermArg, Display, NameString, Result, Stream,
};

pub(in crate::aml::term_objects) struct BankField {
    region_name: NameString,
    bank_name: NameString,
    bank_value: TermArg,
    field_flags: FieldFlags,
    field_list: FieldList,
}

impl BankField {
    pub(super) fn parse(stream: &mut Stream) -> Result<Self> {
        let mut stream = pkg_length::parse_to_stream(stream)?;

        let region_name = NameString::parse(&mut stream)?;
        let bank_name = NameString::parse(&mut stream)?;
        let bank_value = TermArg::parse(&mut stream)?;
        let field_flags = FieldFlags::parse(&mut stream)?;
        let field_list = FieldList::parse(&mut stream)?;

        Ok(BankField {
            region_name,
            bank_name,
            bank_value,
            field_flags,
            field_list,
        })
    }
}

impl Display for BankField {
    fn display(&self, f: &mut core::fmt::Formatter, depth: usize, last: bool) -> core::fmt::Result {
        self.display_prefix(f, depth)?;
        write!(
            f,
            "Bank Field ({}, {}, {}, {}) ",
            self.region_name, self.bank_name, self.bank_value, self.field_flags
        )?;

        self.field_list.display(f, depth, last)
    }
}

impl_core_display!(BankField);
