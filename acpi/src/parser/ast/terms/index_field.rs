use crate::{
    display_prefix, impl_core_display,
    namespace::objects::FieldFlags,
    parser::{name_string, next, pkg_length, Result, Stream},
    Display, Path,
};

pub(crate) struct IndexField {
    path: Path,
    data_name: Path,
    flags: FieldFlags,
    // TODO: Add FieldList
}

impl IndexField {
    pub(super) fn parse(stream: &mut Stream) -> Result<Self> {
        let mut stream = pkg_length::parse_to_stream(stream, "Field")?;

        let path = name_string::parse(&mut stream, "Field")?;
        let data_name = name_string::parse(&mut stream, "Field")?;
        let flags = FieldFlags::parse(next!(stream, "Field"));

        Ok(IndexField {
            path,
            data_name,
            flags,
        })
    }
}

impl Display for IndexField {
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
            "IndexField ({}, {}, {})",
            self.path, self.data_name, self.flags
        )?;

        if newline {
            writeln!(f)
        } else {
            Ok(())
        }
    }
}

impl_core_display!(IndexField);
