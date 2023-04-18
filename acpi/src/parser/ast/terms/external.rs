use crate::{
    display_prefix, impl_core_display,
    parser::{name_string, next, Context, Result, Stream},
    Display, Path,
};

pub(crate) struct External {
    path: Path,
    object_type: u8,
    argument_count: u8,
}

impl External {
    pub(super) fn parse(stream: &mut Stream, context: &mut Context) -> Result<Self> {
        let path = name_string::parse(stream, "External")?;
        let object_type = next!(stream, "External");
        let argument_count = next!(stream, "External");

        context.add_method(&path, argument_count, stream.offset())?;

        Ok(External {
            path,
            object_type,
            argument_count,
        })
    }
}

impl Display for External {
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
            "External ({}, {}, {})",
            self.path, self.object_type, self.argument_count
        )?;

        if newline {
            writeln!(f)
        } else {
            Ok(())
        }
    }
}

impl_core_display!(External);
