use crate::parser::{next, Argument, NameString, Result, Stream};
use region_space::RegionSpace;

mod region_space;

pub(crate) struct OpRegion {
    name: NameString,
    region_space: RegionSpace,
    offset: Argument,
    length: Argument,
}

impl OpRegion {
    pub(super) fn parse(stream: &mut Stream) -> Result<Self> {
        let name = NameString::parse(stream)?;
        let region_space = RegionSpace::from_u8(next!(stream));
        let offset = Argument::parse(stream)?;
        let length = Argument::parse(stream)?;

        Ok(OpRegion {
            name,
            region_space,
            offset,
            length,
        })
    }

    pub(crate) fn name(&self) -> &NameString {
        &self.name
    }

    pub(crate) fn region_space(&self) -> RegionSpace {
        self.region_space
    }

    pub(crate) fn offset(&self) -> &Argument {
        &self.offset
    }

    pub(crate) fn length(&self) -> &Argument {
        &self.length
    }
}
