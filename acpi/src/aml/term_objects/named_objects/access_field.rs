use super::AccessType;
use crate::aml::{impl_core_display, next, Display, Result, Stream};

pub(super) enum AccessAttribClass {
    Normal,
    Bytes,
    RawBytes,
    RawProcessBytes,
}

pub(super) struct AccessField {
    offset: usize,
    access_type: AccessType,
    access_attrib_class: AccessAttribClass,
    access_attrib: u8,
}

impl AccessField {
    pub(super) fn parse(stream: &mut Stream) -> Result<Self> {
        let offset = stream.offset();

        let access_type_byte = next!(stream);
        let access_type = AccessType::parse(access_type_byte & 0xF);
        let access_attrib_class = AccessAttribClass::parse(access_type_byte.wrapping_shr(6) & 3);

        let access_attrib = next!(stream);

        Ok(AccessField {
            offset,
            access_type,
            access_attrib_class,
            access_attrib,
        })
    }
}

impl Display for AccessField {
    fn display(&self, f: &mut core::fmt::Formatter, depth: usize, _: bool) -> core::fmt::Result {
        self.display_prefix(f, depth)?;
        writeln!(
            f,
            "Access Field {} - {} - {:#02X} @ {}",
            self.access_type, self.access_attrib_class, self.access_attrib, self.offset
        )
    }
}

impl_core_display!(AccessField);

impl AccessAttribClass {
    pub(super) fn parse(access_attrib_class: u8) -> Self {
        match access_attrib_class {
            1 => AccessAttribClass::Bytes,
            2 => AccessAttribClass::RawBytes,
            3 => AccessAttribClass::RawProcessBytes,
            _ => AccessAttribClass::Normal,
        }
    }
}

impl core::fmt::Display for AccessAttribClass {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                AccessAttribClass::Normal => "Normal",
                AccessAttribClass::Bytes => "Bytes",
                AccessAttribClass::RawBytes => "Raw Bytes",
                AccessAttribClass::RawProcessBytes => "Raw Process Bytes",
            }
        )
    }
}
