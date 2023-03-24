use crate::aml::{
    impl_core_display, next, pkg_length, term_objects::TermList, Display, NameString, Result,
    Stream,
};

pub(in crate::aml::term_objects) struct PowerRes {
    name: NameString,
    system_level: u8,
    resource_order: u16,
    term_list: TermList,
}

impl PowerRes {
    pub(super) fn parse(stream: &mut Stream) -> Result<Self> {
        let mut stream = pkg_length::parse_to_stream(stream)?;

        let name = NameString::parse(&mut stream)?;
        let system_level = next!(stream);
        let resource_order = u16::from_le_bytes([next!(stream), next!(stream)]);
        let term_list = TermList::parse(&mut stream)?;

        Ok(PowerRes {
            name,
            system_level,
            resource_order,
            term_list,
        })
    }
}

impl Display for PowerRes {
    fn display(&self, f: &mut core::fmt::Formatter, depth: usize, last: bool) -> core::fmt::Result {
        self.display_prefix(f, depth)?;
        write!(
            f,
            "Power Resource ({}, {}, {}) ",
            self.name, self.system_level, self.resource_order
        )?;

        self.term_list.display(f, depth, last)
    }
}

impl_core_display!(PowerRes);
