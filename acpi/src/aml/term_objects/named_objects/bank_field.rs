use super::{field_list::FieldList, FieldFlags};
use crate::aml::{
    impl_core_display, pkg_length, term_objects::TermArg, Display, NameString, Result, Stream,
};

pub(in crate::aml::term_objects) struct BankField {
    offset: usize,
    name1: NameString,
    name2: NameString,
    bank_value: TermArg,
    field_flags: FieldFlags,
    field_list: FieldList,
}

impl BankField {
    pub(super) fn parse(stream: &mut Stream) -> Result<Self> {
        let offset = stream.offset() - 2;

        let mut stream = pkg_length::parse_to_stream(stream)?;

        let name1 = NameString::parse(&mut stream)?;
        let name2 = NameString::parse(&mut stream)?;
        let bank_value = TermArg::parse(&mut stream)?;
        let field_flags = FieldFlags::parse(&mut stream)?;
        let field_list = FieldList::parse(&mut stream)?;

        Ok(BankField {
            offset,
            name1,
            name2,
            bank_value,
            field_flags,
            field_list,
        })
    }
}

impl Display for BankField {
    fn display(&self, f: &mut core::fmt::Formatter, depth: usize) -> core::fmt::Result {
        self.display_prefix(f, depth)?;
        writeln!(
            f,
            "BankField {} -> {} @ {}",
            self.name1, self.name2, self.offset
        )?;

        self.display_prefix(f, depth + 1)?;
        writeln!(f, "Bank Value:")?;

        self.bank_value.display(f, depth + 2)?;

        self.display_prefix(f, depth + 1)?;
        writeln!(f, "Field Flags: {}", self.field_flags)?;

        self.field_list.display(f, depth + 1)
    }
}

impl_core_display!(BankField);
