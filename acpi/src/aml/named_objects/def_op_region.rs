use super::RegionSpace;
use crate::aml::{ASTNode, ByteStream, NameString, Result, TermArg};

pub(in crate::aml) struct DefOpRegion {
    name: NameString,
    region_space: RegionSpace,
    region_offset: TermArg,
    region_length: TermArg,
}

impl DefOpRegion {
    pub(in crate::aml) fn parse(stream: &mut ByteStream) -> Result<Self> {
        let name = NameString::parse(stream)?;
        let region_space = RegionSpace::parse(stream)?;
        let region_offset = TermArg::parse(stream)?;
        let region_length = TermArg::parse(stream)?;

        Ok(DefOpRegion {
            name,
            region_space,
            region_offset,
            region_length,
        })
    }
}

impl ASTNode for DefOpRegion {
    fn display(&self, f: &mut core::fmt::Formatter, depth: usize) -> core::fmt::Result {
        self.prefix_depth(f, depth)?;
        writeln!(f, "Op Region \"{}\":", self.name)?;

        self.prefix_depth(f, depth + 1)?;
        writeln!(f, "Region Space: {}", self.region_space)?;

        self.prefix_depth(f, depth + 1)?;
        writeln!(f, "Region Offset:")?;
        self.region_offset.display(f, depth + 2)?;

        self.prefix_depth(f, depth + 1)?;
        writeln!(f, "Region Length:")?;
        self.region_length.display(f, depth + 2)
    }
}
