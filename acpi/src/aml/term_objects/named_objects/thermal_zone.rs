use crate::aml::{
    impl_core_display, pkg_length, term_objects::TermList, Display, NameString, Result, Stream,
};

pub(in crate::aml::term_objects) struct ThermalZone {
    offset: usize,
    name: NameString,
    term_list: TermList,
}

impl ThermalZone {
    pub(super) fn parse(stream: &mut Stream) -> Result<Self> {
        let offset = stream.offset() - 2;

        let mut stream = pkg_length::parse_to_stream(stream)?;

        let name = NameString::parse(&mut stream)?;
        let term_list = TermList::parse(&mut stream)?;

        Ok(ThermalZone {
            offset,
            name,
            term_list,
        })
    }
}

impl Display for ThermalZone {
    fn display(&self, f: &mut core::fmt::Formatter, depth: usize) -> core::fmt::Result {
        self.display_prefix(f, depth)?;
        writeln!(f, "Thermal Zone {} @ {}", self.name, self.offset)?;

        self.term_list.display(f, depth + 1)
    }
}

impl_core_display!(ThermalZone);
