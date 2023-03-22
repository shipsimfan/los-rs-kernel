use crate::aml::{
    impl_core_display, next, term_objects::TermArg, Display, NameString, Result, Stream,
};
use region_space::RegionSpace;

mod region_space;

pub(in crate::aml::term_objects) struct OpRegion {
    offset: usize,
    name: NameString,

    region_space: RegionSpace,
    region_offset: TermArg,
    region_length: TermArg,
}

impl OpRegion {
    pub(super) fn parse(stream: &mut Stream) -> Result<Self> {
        let offset = stream.offset() - 2;

        let name = NameString::parse(stream)?;
        let region_space = RegionSpace::from_u8(next!(stream));

        let region_offset = TermArg::parse(stream)?;
        let region_length = TermArg::parse(stream)?;

        Ok(OpRegion {
            offset,
            name,

            region_space,
            region_offset,
            region_length,
        })
    }
}

impl Display for OpRegion {
    fn display(&self, f: &mut core::fmt::Formatter, depth: usize) -> core::fmt::Result {
        self.display_prefix(f, depth)?;
        writeln!(f, "OpRegion {} @ {}", self.name, self.offset)?;

        self.display_prefix(f, depth + 1)?;
        writeln!(f, "Region Space: {}", self.region_space)?;

        self.display_prefix(f, depth + 1)?;
        writeln!(f, "Region Offset:")?;
        self.region_offset.display(f, depth + 2)?;

        self.display_prefix(f, depth + 1)?;
        writeln!(f, "Region Length:")?;
        self.region_length.display(f, depth + 2)
    }
}

impl_core_display!(OpRegion);
