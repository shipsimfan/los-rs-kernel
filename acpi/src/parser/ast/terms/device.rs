use super::TermList;
use crate::{
    display_prefix, impl_core_display_lifetime,
    parser::{name_string, pkg_length, Context, Result, Stream},
    Display, Path,
};

pub(crate) struct Device<'a> {
    path: Path,
    term_list: TermList<'a>,
}

impl<'a> Device<'a> {
    pub(super) fn parse(stream: &mut Stream<'a>, context: &mut Context) -> Result<Self> {
        let mut stream = pkg_length::parse_to_stream(stream, "Device")?;

        let path = name_string::parse(&mut stream, "Device")?;

        context.push_path(&path);
        let term_list = TermList::parse(&mut stream, context)?;
        context.pop_path();

        Ok(Device { path, term_list })
    }
}

impl<'a> Display for Device<'a> {
    fn display(&self, f: &mut core::fmt::Formatter, depth: usize, last: bool) -> core::fmt::Result {
        display_prefix!(f, depth);
        write!(f, "Device ({}) ", self.path)?;
        self.term_list.display(f, depth, last)
    }
}

impl_core_display_lifetime!(Device);
