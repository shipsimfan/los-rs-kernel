use crate::parser::{next, Argument, Error, NameString, Result, Stream};

mod region_space;

pub(crate) use region_space::RegionSpace;

pub(crate) struct OpRegion {
    name: NameString,
    region_space: RegionSpace,
    offset: Argument,
    length: Argument,
}

impl OpRegion {
    pub(super) fn parse(stream: &mut Stream) -> Result<Self> {
        let name = NameString::parse(stream)?;
        let region_space = next!(stream);
        let region_space = RegionSpace::from_u8(region_space)
            .ok_or(Error::unexpected_byte(region_space, stream.offset() - 1))?;
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
