use crate::aml::{
    impl_core_display, pkg_length, term_objects::TermList, Display, NameString, Result, Stream,
};

pub(in crate::aml::term_objects) struct Device {
    name: NameString,
    term_list: TermList,
}

impl Device {
    pub(super) fn parse(stream: &mut Stream) -> Result<Self> {
        let mut stream = pkg_length::parse_to_stream(stream)?;

        let name = NameString::parse(&mut stream)?;
        let term_list = TermList::parse(&mut stream)?;

        Ok(Device { name, term_list })
    }
}

impl Display for Device {
    fn display(&self, f: &mut core::fmt::Formatter, depth: usize, last: bool) -> core::fmt::Result {
        self.display_prefix(f, depth)?;
        write!(f, "Device ({}) ", self.name)?;

        self.term_list.display(f, depth, last)
    }
}

impl_core_display!(Device);