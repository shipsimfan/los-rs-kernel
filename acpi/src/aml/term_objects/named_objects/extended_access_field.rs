use super::{AccessAttribClass, AccessType};
use crate::aml::{impl_core_display, next, Display, Result, Stream};

pub(super) struct ExtendedAccessField {
    offset: usize,
    access_type: AccessType,
    access_attrib_class: AccessAttribClass,
    extended_access_attrib: u8,
}

impl ExtendedAccessField {
    #[allow(unused_variables)]
    pub(super) fn parse(stream: &mut Stream) -> Result<Self> {
        let offset = stream.offset();

        let access_type_byte = next!(stream);
        let access_type = AccessType::parse(access_type_byte & 0xF);
        let access_attrib_class = AccessAttribClass::parse(access_type_byte.wrapping_shr(6) & 3);

        let extended_access_attrib = next!(stream);

        todo!("Extended Access Field Access Length");
    }
}

impl Display for ExtendedAccessField {
    fn display(&self, f: &mut core::fmt::Formatter, depth: usize, _: bool) -> core::fmt::Result {
        self.display_prefix(f, depth)?;
        writeln!(
            f,
            "Access Field {} - {} - {:#02X} @ {}",
            self.access_type, self.access_attrib_class, self.extended_access_attrib, self.offset
        )
    }
}

impl_core_display!(ExtendedAccessField);
