use crate::{
    display_prefix, impl_core_display_lifetime,
    parser::{name_string, next, pkg_length, Context, Result, Stream},
    Display, Path,
};
use core::marker::PhantomData;

pub(crate) struct Method<'a> {
    path: Path,
    argument_count: u8,
    serialized: bool,
    sync_level: u8,
    // TODO: Add TermList
    phantom: PhantomData<&'a ()>,
}

fn parse_method_flags(flags: u8) -> (u8, bool, u8) {
    let argument_count = flags & 7;
    let serialized = (flags & 8) == 8;
    let sync_level = flags.wrapping_shr(4) & 0xF;

    (argument_count, serialized, sync_level)
}

impl<'a> Method<'a> {
    pub(super) fn parse(stream: &mut Stream<'a>, context: &mut Context) -> Result<Self> {
        let mut stream = pkg_length::parse_to_stream(stream, "Method")?;

        let offset = stream.offset();
        let path = name_string::parse(&mut stream, "Method")?;
        let (argument_count, serialized, sync_level) = parse_method_flags(next!(stream, "Method"));

        context.add_method(&path, argument_count, offset)?;

        Ok(Method {
            path,
            argument_count,
            serialized,
            sync_level,
            phantom: PhantomData,
        })
    }
}

impl<'a> Display for Method<'a> {
    fn display(&self, f: &mut core::fmt::Formatter, depth: usize, _: bool) -> core::fmt::Result {
        display_prefix!(f, depth);
        writeln!(
            f,
            "Method ({}, {}, {}, {})",
            self.path, self.argument_count, self.serialized, self.sync_level
        )
    }
}

impl_core_display_lifetime!(Method);
