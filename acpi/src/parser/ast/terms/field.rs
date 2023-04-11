use crate::{
    display_prefix, impl_core_display,
    namespace::objects::FieldFlags,
    parser::{name_string, next, pkg_length, Result, Stream},
    Display, Path,
};

pub(crate) struct Field {
    path: Path,
    flags: FieldFlags,
    // TODO: Add FieldList
}

impl Field {
    pub(super) fn parse(stream: &mut Stream) -> Result<Self> {
        let mut stream = pkg_length::parse_to_stream(stream, "Field")?;

        let path = name_string::parse(&mut stream, "Field")?;
        let flags = FieldFlags::parse(next!(stream, "Field"));

        Ok(Field { path, flags })
    }
}

impl Display for Field {
    fn display(&self, f: &mut core::fmt::Formatter, depth: usize, _: bool) -> core::fmt::Result {
        display_prefix!(f, depth);
        writeln!(f, "Field ({}, {})", self.path, self.flags)
    }
}

impl_core_display!(Field);
