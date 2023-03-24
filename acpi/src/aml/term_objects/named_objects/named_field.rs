use crate::aml::{impl_core_display, name_string, pkg_length, Display, Result, Stream};

pub(super) struct NamedField {
    offset: usize,
    name: [u8; 4],
}

impl NamedField {
    pub(super) fn parse(stream: &mut Stream) -> Result<Self> {
        let offset = stream.offset();

        let name = name_string::parse_name_seg(stream, None)?;

        pkg_length::parse_to_stream(stream)?;

        Ok(NamedField { offset, name })
    }
}

impl Display for NamedField {
    fn display(&self, f: &mut core::fmt::Formatter, depth: usize, _: bool) -> core::fmt::Result {
        self.display_prefix(f, depth)?;
        writeln!(
            f,
            "Named Field \"{}{}{}{}\" @ {}",
            self.name[0] as char,
            self.name[1] as char,
            self.name[2] as char,
            self.name[3] as char,
            self.offset
        )
    }
}

impl_core_display!(NamedField);
