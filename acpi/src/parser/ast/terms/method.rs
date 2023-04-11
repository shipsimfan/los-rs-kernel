use super::TermList;
use crate::{
    display_prefix, impl_core_display_lifetime,
    parser::{name_string, next, pkg_length, Context, Result, Stream},
    Display, Path,
};

pub(crate) struct Method<'a> {
    path: Path,
    argument_count: u8,
    serialized: bool,
    sync_level: u8,
    term_list: TermList<'a>,
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

        context.push_path(&path);
        let term_list = TermList::parse(&mut stream, context)?;
        context.pop_path();

        Ok(Method {
            path,
            argument_count,
            serialized,
            sync_level,
            term_list,
        })
    }
}

impl<'a> Display for Method<'a> {
    fn display(&self, f: &mut core::fmt::Formatter, depth: usize, last: bool) -> core::fmt::Result {
        display_prefix!(f, depth);
        write!(
            f,
            "Method ({}, {}, {}, {}) ",
            self.path, self.argument_count, self.serialized, self.sync_level
        )?;
        self.term_list.display(f, depth, last)
    }
}

impl_core_display_lifetime!(Method);
