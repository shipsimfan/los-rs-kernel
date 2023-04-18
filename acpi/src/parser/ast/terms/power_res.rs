use super::TermList;
use crate::{
    display_prefix, impl_core_display_lifetime,
    parser::{name_string, next, pkg_length, Context, Result, Stream},
    Display, Path,
};

pub(crate) struct PowerRes<'a> {
    path: Path,
    system_level: u8,
    resource_order: u16,
    term_list: TermList<'a>,
}

impl<'a> PowerRes<'a> {
    pub(super) fn parse(stream: &mut Stream<'a>, context: &mut Context) -> Result<Self> {
        let mut stream = pkg_length::parse_to_stream(stream, "Power Resource")?;

        let path = name_string::parse(&mut stream, "Power Resource")?;
        let system_level = next!(stream, "Power Resource");
        let resource_order = u16::from_le_bytes([
            next!(stream, "Power Resource"),
            next!(stream, "Power Resource"),
        ]);

        context.push_path(&path);
        let term_list = TermList::parse(&mut stream, context)?;
        context.pop_path();

        Ok(PowerRes {
            path,
            system_level,
            resource_order,
            term_list,
        })
    }
}

impl<'a> Display for PowerRes<'a> {
    fn display(
        &self,
        f: &mut core::fmt::Formatter,
        depth: usize,
        last: bool,
        newline: bool,
    ) -> core::fmt::Result {
        display_prefix!(f, depth);
        write!(
            f,
            "PowerRes ({}, {}, {}) ",
            self.path, self.system_level, self.resource_order
        )?;
        self.term_list.display(f, depth, last, newline)
    }
}

impl_core_display_lifetime!(PowerRes);
