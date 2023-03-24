use crate::aml::{
    impl_core_display, next, term_objects::TermArg, Display, NameString, Result, Stream,
};
use region_space::RegionSpace;

mod region_space;

pub(in crate::aml::term_objects) struct OpRegion {
    name: NameString,
    region_space: RegionSpace,
    offset: TermArg,
    length: TermArg,
}

impl OpRegion {
    pub(super) fn parse(stream: &mut Stream) -> Result<Self> {
        let name = NameString::parse(stream)?;
        let region_space = RegionSpace::from_u8(next!(stream));

        let offset = TermArg::parse(stream)?;
        let length = TermArg::parse(stream)?;

        Ok(OpRegion {
            name,
            region_space,
            offset,
            length,
        })
    }
}

impl Display for OpRegion {
    fn display(&self, f: &mut core::fmt::Formatter, depth: usize, _: bool) -> core::fmt::Result {
        self.display_prefix(f, depth)?;
        writeln!(
            f,
            "Opeation Region ({}, {}, {}, {})",
            self.name, self.region_space, self.offset, self.length
        )
    }
}

impl_core_display!(OpRegion);
