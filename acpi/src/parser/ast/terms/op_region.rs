use crate::{
    display_prefix, impl_core_display_lifetime,
    namespace::objects::RegionSpace,
    parser::{ast::Argument, match_next, name_string, Context, Result, Stream},
    Display, Path,
};

pub(crate) struct OpRegion<'a> {
    path: Path,
    space: RegionSpace,
    offset: Argument<'a>,
    length: Argument<'a>,
}

fn parse_region_space(stream: &mut Stream) -> Result<RegionSpace> {
    Ok(match_next!(stream, "Op Region",
        0x00 => RegionSpace::SystemMemory,
        0x01 => RegionSpace::SystemIO,
        0x02 => RegionSpace::PCIConfig,
        0x03 => RegionSpace::EmbeddedControl,
        0x04 => RegionSpace::SMBus,
        0x05 => RegionSpace::SystemCMOS,
        0x06 => RegionSpace::PCIBarTarget,
        0x07 => RegionSpace::IPMI,
        0x08 => RegionSpace::GeneralPurposeIO,
        0x09 => RegionSpace::GenericSerialBus,
        0x0A => RegionSpace::PCC,
    ))
}

impl<'a> OpRegion<'a> {
    pub(super) fn parse(stream: &mut Stream<'a>, context: &mut Context) -> Result<Self> {
        let path = name_string::parse(stream, "Op Region")?;
        let space = parse_region_space(stream)?;
        let offset = Argument::parse(stream, context)?;
        let length = Argument::parse(stream, context)?;

        Ok(OpRegion {
            path,
            space,
            offset,
            length,
        })
    }
}

impl<'a> Display for OpRegion<'a> {
    fn display(
        &self,
        f: &mut core::fmt::Formatter,
        depth: usize,
        _: bool,
        newline: bool,
    ) -> core::fmt::Result {
        display_prefix!(f, depth);
        write!(
            f,
            "OpRegion ({}, {}, {}, {})",
            self.path, self.space, self.offset, self.length
        )?;

        if newline {
            writeln!(f)
        } else {
            Ok(())
        }
    }
}

impl_core_display_lifetime!(OpRegion);
